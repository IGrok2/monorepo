/*
Copyright 2024.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

package controller

import (
	"context"
	"encoding/base64"
	"fmt"

	appsv1 "k8s.io/api/apps/v1"
	corev1 "k8s.io/api/core/v1"
	networkingv1 "k8s.io/api/networking/v1"
	v1 "k8s.io/api/networking/v1"
	"k8s.io/apimachinery/pkg/api/errors"
	metav1 "k8s.io/apimachinery/pkg/apis/meta/v1"
	"k8s.io/apimachinery/pkg/runtime"
	"k8s.io/apimachinery/pkg/types" // get intstr
	"k8s.io/apimachinery/pkg/util/intstr"
	"k8s.io/utils/pointer"
	ctrl "sigs.k8s.io/controller-runtime"
	"sigs.k8s.io/controller-runtime/pkg/client"
	"sigs.k8s.io/controller-runtime/pkg/controller/controllerutil"
	"sigs.k8s.io/controller-runtime/pkg/log"

	"github.com/go-logr/logr"
	packetwareappv1 "github.com/packetware/app-operator/api/v1"
)

// AppReconciler reconciles a App object
type AppReconciler struct {
	client.Client
	Scheme *runtime.Scheme
}

//+kubebuilder:rbac:groups=packetwareapp.apps.packetware.net,resources=apps,verbs=get;list;watch;create;update;patch;delete
//+kubebuilder:rbac:groups=packetwareapp.apps.packetware.net,resources=apps/status,verbs=get;update;patch
//+kubebuilder:rbac:groups=packetwareapp.apps.packetware.net,resources=apps/finalizers,verbs=update
//+kubebuilder:rbac:groups=apps,resources=deployments,verbs=get;list;watch;create;update;patch;delete
//+kubebuilder:rbac:groups="",resources=pods,verbs=get;list;watch;create;update;patch;delete
//+kubebuilder:rbac:groups="",resources=services,verbs=get;list;watch;create;update;patch;delete
//+kubebuilder:rbac:groups=networking.k8s.io,resources=ingresses,verbs=get;list;watch;create;update;patch;delete
//+kubebuilder:rbac:groups=networking.k8s.io,resources=ingresses/status,verbs=get;update;patch
//+kubebuilder:rbac:groups="",resources=secrets,verbs=get;list;watch;create;update;patch;delete
//+kubebuilder:rbac:groups=networking.k8s.io,resources=networkpolicies,verbs=get;list;watch;create;update;patch;delete

const (
	DefaultRegistryURL      = "https://registry.aqueous.cloud"
	DefaultRegistryUsername = "apps"
	DefaultRegistryPassword = "38U7j3qaozMtTS"
)

// Reconcile is part of the main kubernetes reconciliation loop which aims to
// move the current state of the cluster closer to the desired state.
// TODO(user): Modify the Reconcile function to compare the state specified by
// the App object against the actual cluster state, and then
// perform operations to make the cluster state reflect the state specified by
// the user.
//
// For more details, check Reconcile and its Result here:
// - https://pkg.go.dev/sigs.k8s.io/controller-runtime@v0.17.3/pkg/reconcile
func (r *AppReconciler) Reconcile(ctx context.Context, req ctrl.Request) (ctrl.Result, error) {
	log := log.FromContext(ctx)

	// Fetch the App instance
	app := &packetwareappv1.App{} // Use the alias for your custom resource
	err := r.Get(ctx, req.NamespacedName, app)
	if err != nil {
		if errors.IsNotFound(err) {
			return ctrl.Result{}, nil
		}
		return ctrl.Result{}, err
	}

	// Use default registry credentials if not specified
	registryURL := DefaultRegistryURL
	registryUsername := DefaultRegistryUsername
	registryPassword := DefaultRegistryPassword

	if app.Spec.Registry.URL != "" {
		registryURL = app.Spec.Registry.URL
	}
	if app.Spec.Registry.Auth.Username != "" {
		registryUsername = app.Spec.Registry.Auth.Username
	}
	if app.Spec.Registry.Auth.Password != "" {
		registryPassword = app.Spec.Registry.Auth.Password
	}

	// Create imagePullSecret
	imagePullSecretName := app.Name + "-registry-secret"
	imagePullSecret := &corev1.Secret{
		ObjectMeta: metav1.ObjectMeta{
			Name:      imagePullSecretName,
			Namespace: app.Namespace,
		},
		Type: corev1.SecretTypeDockerConfigJson,
		Data: map[string][]byte{
			".dockerconfigjson": []byte(fmt.Sprintf(`{
					"auths": {
						"%s": {
							"username": "%s",
							"password": "%s",
							"auth": "%s"
						}
					}
				}`, registryURL, registryUsername, registryPassword,
				base64.StdEncoding.EncodeToString([]byte(registryUsername+":"+registryPassword)))),
		},
	}

	// Set App instance as the owner and controller
	if err := controllerutil.SetControllerReference(app, imagePullSecret, r.Scheme); err != nil {
		return ctrl.Result{}, err
	}

	// Check if the imagePullSecret already exists
	foundSecret := &corev1.Secret{}
	err = r.Get(ctx, types.NamespacedName{Name: imagePullSecret.Name, Namespace: imagePullSecret.Namespace}, foundSecret)
	if err != nil && errors.IsNotFound(err) {
		log.Info("Creating a new imagePullSecret", "Secret.Namespace", imagePullSecret.Namespace, "Secret.Name", imagePullSecret.Name)
		err = r.Create(ctx, imagePullSecret)
		if err != nil {
			return ctrl.Result{}, err
		}
	} else if err != nil {
		return ctrl.Result{}, err
	} else {
		// Update the existing secret if needed
		foundSecret.Data = imagePullSecret.Data
		log.Info("Updating imagePullSecret", "Secret.Namespace", foundSecret.Namespace, "Secret.Name", foundSecret.Name)
		err = r.Update(ctx, foundSecret)
		if err != nil {
			return ctrl.Result{}, err
		}
	}

	// Create a slice to hold the volume mounts
	volumeMounts := []corev1.VolumeMount{}
	volumes := []corev1.Volume{}

	// Add each volume to the volumes and volumeMounts slices
	for _, volumeConfig := range app.Spec.Volumes {
		volume := corev1.Volume{
			Name: volumeConfig.Name,
			VolumeSource: corev1.VolumeSource{
				PersistentVolumeClaim: &corev1.PersistentVolumeClaimVolumeSource{
					ClaimName: volumeConfig.Name,
				},
			},
		}
		volumes = append(volumes, volume)

		volumeMount := corev1.VolumeMount{
			Name:      volumeConfig.Name,
			MountPath: volumeConfig.MountPath,
		}
		volumeMounts = append(volumeMounts, volumeMount)
	}

	// Create a slice to hold the container ports
	containerPorts := []corev1.ContainerPort{}
	for _, portConfig := range app.Spec.Ports {
		containerPort := corev1.ContainerPort{
			//Name:          portConfig.Name,
			ContainerPort: portConfig.Port,
			//Protocol:      corev1.Protocol(portConfig.Protocol),
		}
		containerPorts = append(containerPorts, containerPort)
	}

	// Define the desired Deployment object
	deploy := &appsv1.Deployment{
		ObjectMeta: metav1.ObjectMeta{
			Name:      app.Name,
			Namespace: app.Namespace,
		},
		Spec: appsv1.DeploymentSpec{
			Replicas: &app.Spec.Replicas,
			Selector: &metav1.LabelSelector{
				MatchLabels: map[string]string{"app": app.Name},
			},
			Template: corev1.PodTemplateSpec{
				ObjectMeta: metav1.ObjectMeta{
					Labels: map[string]string{"app": app.Name},
				},
				Spec: corev1.PodSpec{
					ImagePullSecrets: []corev1.LocalObjectReference{
						{
							Name: imagePullSecretName,
						},
					},
					Containers: []corev1.Container{
						{
							Name:         "app",
							Image:        app.Spec.Image,
							Args:         app.Spec.Args,
							Resources:    app.Spec.Resources,
							Ports:        containerPorts,
							Env:          app.Spec.Env,
							VolumeMounts: volumeMounts,
						},
					},
					Volumes: volumes,
				},
			},
		},
	}

	// Set App instance as the owner and controller
	if err := controllerutil.SetControllerReference(app, deploy, r.Scheme); err != nil {
		return ctrl.Result{}, err
	}

	// Check if the Deployment already exists
	found := &appsv1.Deployment{}
	err = r.Get(ctx, types.NamespacedName{Name: deploy.Name, Namespace: deploy.Namespace}, found)
	if err != nil && errors.IsNotFound(err) {
		log.Info("Creating a new Deployment", "Deployment.Namespace", deploy.Namespace, "Deployment.Name", deploy.Name)
		err = r.Create(ctx, deploy)
		if err != nil {
			return ctrl.Result{}, err
		}
	} else if err != nil {
		return ctrl.Result{}, err
	} else {
		// Update the existing Deployment
		found.Spec = deploy.Spec
		log.Info("Updating Deployment", "Deployment.Namespace", found.Namespace, "Deployment.Name", found.Name)
		err = r.Update(ctx, found)
		if err != nil {
			return ctrl.Result{}, err
		}
	}

	// Call the functi// Call the function to reconcile the NetworkPolicy
	if err := r.reconcileNetworkPolicy(ctx, log, app); err != nil {
		return ctrl.Result{}, err
	}

	serviceType := corev1.ServiceTypeNodePort
	if app.Spec.Private {
		serviceType = corev1.ServiceTypeClusterIP
	}

	svc := &corev1.Service{
		ObjectMeta: metav1.ObjectMeta{
			Name:      app.Name,
			Namespace: app.Namespace,
			Labels:    map[string]string{"app": app.Name},
		},
		Spec: corev1.ServiceSpec{
			Selector: map[string]string{"app": app.Name},
			Ports:    app.Spec.Ports,
			Type:     serviceType,
		},
	}

	// Set App instance as the owner and controller
	if err := controllerutil.SetControllerReference(app, svc, r.Scheme); err != nil {
		return ctrl.Result{}, err
	}

	// Check if the Service already exists
	foundSvc := &corev1.Service{}
	err = r.Get(ctx, types.NamespacedName{Name: svc.Name, Namespace: svc.Namespace}, foundSvc)
	if err != nil && errors.IsNotFound(err) {
		log.Info("Creating a new Service", "Service.Namespace", svc.Namespace, "Service.Name", svc.Name)
		err = r.Create(ctx, svc)
		if err != nil {
			return ctrl.Result{}, err
		}
	} else if err != nil {
		return ctrl.Result{}, err
	} else {
		// Update the existing Service
		foundSvc.Spec = svc.Spec
		log.Info("Updating Service", "Service.Namespace", foundSvc.Namespace, "Service.Name", foundSvc.Name)
		err = r.Update(ctx, foundSvc)
		if err != nil {
			return ctrl.Result{}, err
		}
	}

	if app.Spec.AppType == "HTTP" {
		host1 := app.Name + ".onpacketware.net"
		host2 := app.Name + ".app.aqueous.cloud"
		port := networkingv1.ServiceBackendPort{
			Number: app.Spec.Ports[0].Port,
		}
		pathType := networkingv1.PathTypePrefix

		ingress := &networkingv1.Ingress{
			ObjectMeta: metav1.ObjectMeta{
				Name:      app.Name,
				Namespace: app.Namespace,
				Labels:    map[string]string{"app": app.Name},
			},
			Spec: networkingv1.IngressSpec{
				IngressClassName: pointer.String("nginx"),
				Rules: []networkingv1.IngressRule{
					{
						Host: host1,
						IngressRuleValue: networkingv1.IngressRuleValue{
							HTTP: &networkingv1.HTTPIngressRuleValue{
								Paths: []networkingv1.HTTPIngressPath{
									{
										Path:     "/",
										PathType: &pathType,
										Backend: networkingv1.IngressBackend{
											Service: &networkingv1.IngressServiceBackend{
												Name: svc.Name,
												Port: port,
											},
										},
									},
								},
							},
						},
					},
					{
						Host: host2,
						IngressRuleValue: networkingv1.IngressRuleValue{
							HTTP: &networkingv1.HTTPIngressRuleValue{
								Paths: []networkingv1.HTTPIngressPath{
									{
										Path:     "/",
										PathType: &pathType,
										Backend: networkingv1.IngressBackend{
											Service: &networkingv1.IngressServiceBackend{
												Name: svc.Name,
												Port: port,
											},
										},
									},
								},
							},
						},
					},
				},
			},
		}

		// Set App instance as the owner and controller
		if err := controllerutil.SetControllerReference(app, ingress, r.Scheme); err != nil {
			return ctrl.Result{}, err
		}

		// Check if the Ingress already exists
		foundIngress := &networkingv1.Ingress{}
		err = r.Get(ctx, types.NamespacedName{Name: ingress.Name, Namespace: ingress.Namespace}, foundIngress)
		if err != nil && errors.IsNotFound(err) {
			log.Info("Creating a new Ingress", "Ingress.Namespace", ingress.Namespace, "Ingress.Name", ingress.Name)
			err = r.Create(ctx, ingress)
			if err != nil {
				return ctrl.Result{}, err
			}
		} else if err != nil {
			return ctrl.Result{}, err
		} else {
			// Update the existing Ingress
			foundIngress.Spec = ingress.Spec
			log.Info("Updating Ingress", "Ingress.Namespace", foundIngress.Namespace, "Ingress.Name", foundIngress.Name)
			err = r.Update(ctx, foundIngress)
			if err != nil {
				return ctrl.Result{}, err
			}
		}
	}

	/*
	 * Network Policies
	 */

	return ctrl.Result{}, nil
}

// SetupWithManager sets up the controller with the Manager.
func (r *AppReconciler) SetupWithManager(mgr ctrl.Manager) error {
	return ctrl.NewControllerManagedBy(mgr).
		For(&packetwareappv1.App{}).
		Owns(&appsv1.Deployment{}).
		Owns(&networkingv1.NetworkPolicy{}).
		Owns(&networkingv1.Ingress{}).
		Owns(&corev1.Service{}).
		Owns(&corev1.Secret{}).
		Complete(r)
}

// Supporting Helpers
func (r *AppReconciler) reconcileNetworkPolicy(ctx context.Context, log logr.Logger, app *packetwareappv1.App) error {
	// Ingress rule placeholder
	networkPolicyIngressRules := []networkingv1.NetworkPolicyIngressRule{}

	// If the app is public we should make these ports open to ingress
	if !app.Spec.Private {
		// Create a slice to hold the networkPolicy ports
		networkPolicyPorts := []networkingv1.NetworkPolicyPort{}

		for _, portConfig := range app.Spec.Ports {
			// Convert port number to IntOrString
			port := intstr.FromInt(int(portConfig.Port))

			// Create NetworkPolicyPort object
			networkPolicyPort := v1.NetworkPolicyPort{
				Port:     &port,
				Protocol: &portConfig.Protocol,
			}
			networkPolicyPorts = append(networkPolicyPorts, networkPolicyPort)
		}

		networkPolicyIngressRule := networkingv1.NetworkPolicyIngressRule{
			Ports: networkPolicyPorts,
		}

		networkPolicyIngressRules = append(networkPolicyIngressRules, networkPolicyIngressRule)
	}

	// Create default NetworkPolicy that blocks everything except service targetPorts
	defaultNetworkPolicy := &networkingv1.NetworkPolicy{
		ObjectMeta: metav1.ObjectMeta{
			Name:      app.Name + "-default",
			Namespace: app.Namespace,
		},
		Spec: networkingv1.NetworkPolicySpec{
			PodSelector: metav1.LabelSelector{
				MatchLabels: map[string]string{"app": app.Name},
			},
			PolicyTypes: []networkingv1.PolicyType{
				networkingv1.PolicyTypeIngress,
			},
			Ingress: networkPolicyIngressRules,
		},
	}

	// Set App instance as the owner and controller
	if err := controllerutil.SetControllerReference(app, defaultNetworkPolicy, r.Scheme); err != nil {
		return err
	}

	// Check if the default NetworkPolicy already exists
	foundDefaultNetworkPolicy := &networkingv1.NetworkPolicy{}
	err := r.Get(ctx, types.NamespacedName{Name: defaultNetworkPolicy.Name, Namespace: defaultNetworkPolicy.Namespace}, foundDefaultNetworkPolicy)
	if err != nil && errors.IsNotFound(err) {
		log.Info("Creating a new default NetworkPolicy", "NetworkPolicy.Namespace", defaultNetworkPolicy.Namespace, "NetworkPolicy.Name", defaultNetworkPolicy.Name)
		err = r.Create(ctx, defaultNetworkPolicy)
		if err != nil {
			return err
		}
	} else if err != nil {
		return err
	} else {
		// Update the existing default NetworkPolicy if needed
		foundDefaultNetworkPolicy.Spec = defaultNetworkPolicy.Spec
		log.Info("Updating default NetworkPolicy", "NetworkPolicy.Namespace", foundDefaultNetworkPolicy.Namespace, "NetworkPolicy.Name", foundDefaultNetworkPolicy.Name)
		err = r.Update(ctx, foundDefaultNetworkPolicy)
		if err != nil {
			return err
		}
	}

	// Create NetworkPolicy for private services to only allow ingress from the same namespace
	privateNetworkPolicy := &networkingv1.NetworkPolicy{
		ObjectMeta: metav1.ObjectMeta{
			Name:      app.Name + "-private",
			Namespace: app.Namespace,
		},
		Spec: networkingv1.NetworkPolicySpec{
			PodSelector: metav1.LabelSelector{
				MatchLabels: map[string]string{"app": app.Name},
			},
			PolicyTypes: []networkingv1.PolicyType{
				networkingv1.PolicyTypeIngress,
			},
			Ingress: []networkingv1.NetworkPolicyIngressRule{
				{
					From: []networkingv1.NetworkPolicyPeer{
						{
							NamespaceSelector: &metav1.LabelSelector{
								MatchLabels: map[string]string{
									"kubernetes.io/metadata.name": app.Namespace,
								},
							},
						},
					},
				},
			},
		},
	}

	// Set App instance as the owner and controller
	if err := controllerutil.SetControllerReference(app, privateNetworkPolicy, r.Scheme); err != nil {
		return err
	}

	// Check if the private NetworkPolicy already exists
	foundPrivateNetworkPolicy := &networkingv1.NetworkPolicy{}
	err = r.Get(ctx, types.NamespacedName{Name: privateNetworkPolicy.Name, Namespace: privateNetworkPolicy.Namespace}, foundPrivateNetworkPolicy)
	if err != nil && errors.IsNotFound(err) {
		log.Info("Creating a new private NetworkPolicy", "NetworkPolicy.Namespace", privateNetworkPolicy.Namespace, "NetworkPolicy.Name", privateNetworkPolicy.Name)
		err = r.Create(ctx, privateNetworkPolicy)
		if err != nil {
			return err
		}
	} else if err != nil {
		return err
	} else {
		// Update the existing private NetworkPolicy if needed
		foundPrivateNetworkPolicy.Spec = privateNetworkPolicy.Spec
		log.Info("Updating private NetworkPolicy", "NetworkPolicy.Namespace", foundPrivateNetworkPolicy.Namespace, "NetworkPolicy.Name", foundPrivateNetworkPolicy.Name)
		err = r.Update(ctx, foundPrivateNetworkPolicy)
		if err != nil {
			return err
		}
	}

	return nil
}

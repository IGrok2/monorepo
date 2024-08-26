contexts=("ash" "dal" "lax")

for context in "${contexts[@]}"; do
  echo "Switching to context $context"
  kubectl config use-context $context

  echo "Deploying app operator to $context"
  make deploy IMG=registry.aqueous.cloud/public/app-operator:0.0.28
done

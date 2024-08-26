/// An API for creating, updating, and deleting apps within gRPC.
/// For HTTP applications, this should be used by the Packetware Operator (within Vulcancore)
/// For TCP applications, this should be moved to the userspace application

package rpc

// / Update an existing app. This should be done in the database post-update hook with the App model.
func UpdateApp() error {
	return nil
}

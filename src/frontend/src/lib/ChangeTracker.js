
// takes array, gets passed the array and the index of the change, and the change itself
// saving this for later, makes more sense to do locally atm
export const calculateChanges = (array, index, change) => {
    // array is the array of objects, index is the desired index, change is the change to make (a JSON object)

    // make a temp array
    let foo = array;

    // get the object at the index
    let object = foo[index];

    // was the opposite change staged?
    // for example, if we're adding an object, was the object already staged for deletion?
    // if so, remove it from the array, and no changes need to be made


}

export const getChanges = (array) => {
    let new_array = [];
    let update_array = [];
    let delete_array = [];

    for (let i = 0; i < array.length; i++) {
        const o = array[i];

        if (o.new_staged) {
            new_array.push(o);
        } else if (o.existed_modified) {
            update_array.push(o);
        } else if (o.being_deleted) {
            delete_array.push(o);
        }
    }

    return {
        new: new_array ?? false,
        updated: update_array ?? false,
        deleted: delete_array ?? false
    }
}

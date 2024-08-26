/* Javascript is fucking doing some odd shit with pass by reference
     * instead of pass by value when I do the following
     * let domain_info = Object.assign({}, domain);
     * so i am now making a function to force pass by value.
     */
export function deepClone(obj) {
    // Check for null or primitive types (return as is)
    if (obj === null || typeof obj !== "object") {
        return obj;
    }

    // Handle Array
    if (Array.isArray(obj)) {
        const arrCopy = [];
        obj.forEach((item, index) => {
            arrCopy[index] = deepClone(item);
        });
        return arrCopy;
    }

    // Handle Object
    const objCopy = {};
    Object.keys(obj).forEach((key) => {
        objCopy[key] = deepClone(obj[key]);
    });
    return objCopy;
}

export default deepClone;
/// a timeout function that takes a promise and a timeout in milliseconds
/// then races the promises against each other
/// if the timeout promise resolves first, the function will return a timeout error
/// if the promise resolves first, the function will return the result of the promise
export async function timeout(
  promise: Promise<any>,
  milliseconds: number,
): Promise<any> {
  return Promise.race([promise, createTimeoutPromise(milliseconds)]);
}

/// a function that takes in number of milliseconds to sleep and returns a promise that will resolve after that time
/// designed to be raced against another promise
function createTimeoutPromise(milliseconds: number): Promise<string> {
  return new Promise((_resolve, reject) => {
    setTimeout(() => {
      reject("timeout excceded");
    }, milliseconds);
  });
}

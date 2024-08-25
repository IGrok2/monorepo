export const notification = async (newMsg, sub) => {
    classes =
        "transform ease-out duration-300 transition translate-y-0 opacity-100 sm:translate-x-0";

    message = newMsg;
    submessage = sub;

    await new Promise((resolve) => setTimeout(resolve, 3000));

    classes = "transition ease-in duration-100 opacity-0";

    message = undefined;
};

export const error_notification = async (newMsg, sub) => {
    classes =
        "transform ease-out duration-300 transition translate-y-0 opacity-100 sm:translate-x-0";

    error = newMsg;
    error_submessage = sub;

    await new Promise((resolve) => setTimeout(resolve, 7000));

    classes = "transition ease-in duration-100 opacity-0";

    error = undefined;
};
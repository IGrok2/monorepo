// for converting bytes to human readable format
export function beautifulBytes(data_transferred) {
    let data_units = "";

    if (data_transferred > 1024 ** 2) {
        data_transferred = data_transferred / 1024 ** 2;
        data_units = "GBs";
    } else if (data_transferred > 1024) {
        data_transferred = data_transferred / 1024;
        data_units = "MBs";
    } else {
        data_units = "KBs";
    }

    return (Math.round(data_transferred * 100) / 100) + data_units;
}
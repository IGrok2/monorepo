// parse a date
// should go as specific as the date if it exists
// for example, if it's 3601 seconds, it should show 1hr 1s
// if it's 90, it should show as 1m 30s
// if it's 0, it should show as 0s
export function beautiful_date(seconds) {
    if (!seconds) {
        return "";
    }

    if (seconds > 604800) {
        return "Too long: max is 7 days";
    }

    const days = Math.floor(seconds / 86400);
    let hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    const remainingSeconds = seconds % 60;

    let result = "";

    if (days > 0) {
        hours -= 24 * days;

        result += `${days}d `;
    }

    if (hours > 0) {
        result += `${hours}hr `;
    }

    if (minutes > 0) {
        result += `${minutes}m `;
    }

    if (remainingSeconds > 0) {
        result += `${remainingSeconds}s`;
    }

    return result;
}

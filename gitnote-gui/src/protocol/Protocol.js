export const sendToIde = (data) => {
    // Check if jsQuery is available
    if (window.sendMessageToIde !== undefined) {
        console.log(`sendMessageToIde = [${window.sendMessageToIde}]`)
        window.sendMessageToIde("reactMessageType", data, "reactMessageId");
    } else {
        console.error('sendMessageToIde is not defined');
    }
};

import { v4 as uuidv4 } from "uuid";

export async function requestToIde(messageType, data) {
    const messageId = uuidv4();

    return new Promise((resolve) => {
        const handler = (event) => {
            if (event.data.messageId === messageId) {
                console.log("requestToIde got data : " + event.data.data);
                window.removeEventListener("message", handler);
                resolve(JSON.parse(event.data.data));
            }
        };
        window.addEventListener("message", handler);

        sendToIde(messageType, data, messageId);
    });
}

const sendToIde = (messageType, data, messageId, attempt = 0) => {
    try {
        sendToIde0(messageType, data, messageId);
    } catch (error) {
        setTimeout(
            () => sendToIde(messageType, data, messageId, attempt + 1),
            Math.pow(2, attempt) * 1000,
        );
    }
};

const sendToIde0 = (messageType, data, messageId) => {
    if (window.sendMessageToIde === undefined) {
        console.log("sendMessageToIde is undefined yet.");
        throw new Error("sendMessageToIde is undefined yet.");
    }
    window.sendMessageToIde(messageType, data, messageId);
}

import { v4 as uuidv4 } from "uuid";

export async function requestToIde(type, data, onError = (msg) => {console.log(msg)}) {
    const id = uuidv4();

    return new Promise((resolve) => {
        const handler = (event) => {
            if (event.data.id === id) {
                window.removeEventListener("message", handler);
                const payload = event.data.payload;
                if (payload.error) {
                    onError(payload.error);
                    return;
                }
                console.log("requestToIde", payload.data);
                resolve(payload.data);
            }
        };
        window.addEventListener("message", handler);

        sendToIde(type, data, id);
    });
}

const sendToIde = (type, payload, id, attempt = 0) => {
    try {
        sendToIde0(type, payload, id);
    } catch (error) {
        if (attempt >= 5) {
            console.error("sendToIde failed after 5 attempts.");
            throw error;
        }
        setTimeout(
            () => sendToIde(type, payload, id, attempt + 1),
            Math.pow(2, attempt) * 1000,
        );
    }
};

const sendToIde0 = (type, payload, id) => {
    if (window.sendMessageToIde === undefined) {
        console.log("sendMessageToIde is undefined yet.");
        throw new Error("sendMessageToIde is undefined yet.");
    }
    window.sendMessageToIde(type, payload, id);
}

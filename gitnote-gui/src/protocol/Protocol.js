import { v4 as uuidv4 } from "uuid";

export async function requestToIde(type, data) {
    const id = uuidv4();

    return new Promise((resolve) => {
        const handler = (event) => {
            if (event.data.id === id) {
                console.log("requestToIde got data : " + id + "..." + event.data.data);
                window.removeEventListener("message", handler);
                const data = event.data.data;
                const ret = data ? JSON.parse(data.replace(/\\/g,'\\')) : "";
                resolve(ret);
            }
        };
        window.addEventListener("message", handler);

        sendToIde(type, data, id);
    });
}

const sendToIde = (type, data, id, attempt = 0) => {
    try {
        sendToIde0(type, data, id);
    } catch (error) {
        if (attempt >= 5) {
            console.error("sendToIde failed after 5 attempts.");
            throw error;
        }
        setTimeout(
            () => sendToIde(type, data, id, attempt + 1),
            Math.pow(2, attempt) * 1000,
        );
    }
};

const sendToIde0 = (type, data, id) => {
    if (window.sendMessageToIde === undefined) {
        console.log("sendMessageToIde is undefined yet.");
        throw new Error("sendMessageToIde is undefined yet.");
    }
    window.sendMessageToIde(type, data, id);
}

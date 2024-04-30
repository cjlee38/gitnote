import Message from "./Message";
import {requestToIde} from "../protocol/Protocol";
import {useEffect, useState} from "react";

class NoteResponse {
    constructor(message) {
        this.message = message;
    }
}

const Note = () => {
    const [messages, setMessages] = useState([]);

    useEffect(() => {
        requestToIde("initialMessages", {})
            .then((data) => {
                console.log("requestToIde got data : " + data);
                handleMessage(data);
            }).catch((error) => {
            console.log("requestToIde got error : " + error);
        });
    }, []);

    // useEffect(() => {sendToIde("initialMessages", {});}, []);
    // useWebViewListener("initialMessages", (data) => handleMessage(data));

    const handleMessage = (data) => {
        console.log(`handleMessage : ${data}`);
        setMessages(data);
        console.log(`the messages : ${messages}`)
    }

    return (
        <div>
            {
                messages.map((message) => (<Message message={message}></Message>))
            }
        </div>
    );
}

export default Note;

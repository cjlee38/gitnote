import Message from "./Message";
import {requestToIde} from "../protocol/Protocol";
import {useEffect, useState} from "react";

const Note = (props) => {
    const [messages, setMessages] = useState([]);

    useEffect(() => {
        requestToIde("initialMessages", {})
            .then((data) => {
                console.log("initialMessages got data : " + data);
                handleMessage(data);
            }).catch((error) => {
            console.log("initialMessages got error : " + error);
        });
    }, []);

    const handleMessage = (data) => {
        console.log(`handleMessage : ${data}`);
        setMessages(data);
        console.log(`the messages : ${messages}`)
    }

    return (
        <div>
            {
                messages.map((message) => (<Message message={message} theme={props.theme}></Message>))
            }
        </div>
    );
}

export default Note;

import Message from "./Message";
import {requestToIde} from "../protocol/Protocol";
import {useEffect, useState} from "react";
import {Button} from "antd";

const Note = (props) => {
    const [messages, setMessages] = useState([]);

    const readMessages = () => requestToIde("messages/read", {})
        .then((data) => {
            console.log("initialMessages got data : " + data);
            handleMessage(data);
        }).catch((error) => {
            console.log("initialMessages got error : " + error);
        });

    useEffect(() => {
        readMessages();
    }, []);

    const handleMessage = (data) => {
        console.log(`handleMessage : ${data}`);
        if (!data.length) {
            console.log("message empty");
        }
        setMessages(data);
        console.log(`the messages : ${messages}`)
    }

    const handleAdd = () => {
        requestToIde("messages/create", {message: ""})
            .then((data) => {
                console.log("addMessage got data : " + data);
                readMessages();
            }).catch((error) => {
            console.log("addMessage got error : " + error);
        });
    }

    return (
        <div>
            {messages && messages.map((message) => (
                <Message
                    message={message}
                    theme={props.theme}
                    reload={readMessages}
                />
            ))},
            {!messages &&
                <Button
                    onClick={() => handleAdd()}
                >Add a new note</Button>}
        </div>
    );
}

export default Note;

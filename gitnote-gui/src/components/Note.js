import Message from "./Message";
import {requestToIde} from "../protocol/Protocol";
import {useEffect, useState} from "react";
import {message as antdMessage} from "antd";

const Note = (props) => {
    const [messages, setMessages] = useState([]);

    const [messageApi, contextHolder] = antdMessage.useMessage();
    useEffect(() => {
        readMessages();
    }, []);

    const readMessages = () => requestToIde("messages/read", {}, messageApi.error)
        .then((data) => {
            console.log("initialMessages got data : " + data);
            handleMessage(data);
        }).catch((error) => {
            console.log("initialMessages got error : " + error);
        });

    const handleMessage = (data) => {
        console.log(`handleMessage : ${JSON.stringify(data)}`);
        setMessages(data);
        console.log(`the messages : ${messages}`)
    }

    return (
        <>
            {contextHolder}
            {messages.map((message) => (
                <Message
                    message={message}
                    theme={props.theme}
                    reload={readMessages}
                />
            ))}
        </>
    );
}

export default Note;

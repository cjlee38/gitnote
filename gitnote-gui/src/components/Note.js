import Message from "./Message";
import {requestToIde} from "../protocol/Protocol";
import {useCallback, useEffect, useRef, useState} from "react";
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

    // const updateDimensions = () => {
    //     console.log("Container dimensions: ", containerRef.current.offsetWidth, containerRef.current.offsetHeight);
    //
    //     requestToIde("window/resize", {
    //         width: containerRef.current.offsetWidth,
    //         height: containerRef.current.offsetHeight,
    //     }, messageApi.error).then((data) => {
    //         console.log("resizeWindow got data : " + data);
    //     });
    // }

    const refCallback = useCallback((node) => {
        if (node) {
            const resizeObserver = new ResizeObserver(() => {
                console.log("Container dimensions: ", node.offsetWidth, node.offsetHeight);

                requestToIde("window/resize", {
                    width: node.offsetWidth,
                    height: node.offsetHeight,
                }, messageApi.error).then((data) => {
                    console.log("resizeWindow got data : " + data);
                });
            });
            resizeObserver.observe(node);

            // Cleanup function to unobserve the node
            return () => {resizeObserver.unobserve(node);};
        }
    }, []);

    return (
        <div
            ref={refCallback}
        >
            {contextHolder}
            {messages.map((message) => (
                <Message
                    message={message}
                    theme={props.theme}
                    reload={readMessages}
                />
            ))}
        </div>
    );
}

export default Note;

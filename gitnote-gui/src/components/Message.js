import {Button, Flex, Space, Tooltip} from "antd";
import {DeleteOutlined, EditOutlined} from "@ant-design/icons";
import TextareaAutosize from 'react-textarea-autosize';
import {useState} from "react";
import {requestToIde} from "../protocol/Protocol";

const Message = (props) => {
    const theme = props.theme;
    const message = props.message;
    const [messageValue, setMessageValue] = useState(message.message);
    const [prevMessageValue, setPrevMessageValue] = useState(message.message);
    const [showIcons, setShowIcons] = useState(false);
    const [isEdit, setIsEdit] = useState(false);

    const handleEdit = () => {
        setIsEdit(true);
    }

    function handleOKClick() {
        setIsEdit(false);
        // console.log(`prevMessage = ${JSON.stringify(prevMessageValue)}`)
        // console.log(`message = ${JSON.stringify(messageValue)}`)
        console.log(`message combine result = ${JSON.stringify(Object.assign({}, message, {message: messageValue}))}`)
        requestToIde("updateMessage", Object.assign({}, message, {message: messageValue}))
            .then((data) => {
                setPrevMessageValue(messageValue);
                setMessageValue(prevMessageValue);
                console.log("updateMessage got data : " + data);
            }).catch((error) => {
            console.log("updateMessage got error : " + error);
        });
    }

    function handleCancelClick() {
        setIsEdit(false);
        setMessageValue(prevMessageValue);
    }

    return (
        <Space direction="vertical">
            <Space direction="horizontal"
                   style={{
                       position: 'relative',
                       display: 'inline-block',
                       width: 'fit-content',
                       border: '1px solid #d9d9d9',
                       borderRadius: '6px',
                       padding: '2px'
                   }}
                   onMouseEnter={() => setShowIcons(true)}
                   onMouseLeave={() => setShowIcons(false)}
            >
                <TextareaAutosize
                    minRows={3}
                    style={{resize: 'none', border: 'none', backgroundColor: theme.editorBackground, color: theme.text}}
                    value={messageValue}
                    onChange={(e) => setMessageValue(e.target.value)}
                    readOnly={!isEdit}
                    wrap="soft"
                    cols={50}
                />
                {showIcons && (
                    <div
                        style={{
                            position: 'absolute',
                            top: '2px',
                            right: '2px',
                            display: 'flex',
                            gap: '4px'
                        }}
                    >
                        <Tooltip title="Edit">
                            <Button
                                size="small"
                                shape="circle"
                                icon={<EditOutlined/>}
                                onClick={() => handleEdit()}
                            />
                        </Tooltip>
                        <Tooltip title="Delete">
                            <Button
                                size="small"
                                shape="circle"
                                icon={<DeleteOutlined/>}
                                onClick={() => alert('Delete')}
                            />
                        </Tooltip>
                    </div>
                )}
            </Space>
            {isEdit && (
                <Flex style={{float: "right", gap: 'large'}}>
                    <Button
                        size="small"
                        type="primary"
                        style={{
                            color: theme.text,
                            backgroundColor: theme.backgroundColor,
                        }}
                        onClick={() => handleOKClick()}
                    >OK</Button>
                    <Button
                        size="small"
                        type="primary"
                        style={{
                            color: theme.text,
                            backgroundColor: "#000033",
                        }}
                        onClick={() => handleCancelClick()}
                    >CANCEL</Button>
                </Flex>
            )}
        </Space>
    );
}

export default Message

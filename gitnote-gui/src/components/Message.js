import {Button, Flex, Popconfirm, Space, Tooltip} from "antd";
import {DeleteOutlined, EditOutlined} from "@ant-design/icons";
import TextareaAutosize from 'react-textarea-autosize';
import {useState} from "react";
import {requestToIde} from "../protocol/Protocol";
import {message as antdMessage} from "antd";

const Message = (props) => {
    const theme = props.theme;
    const message = props.message;
    const [messageValue, setMessageValue] = useState(message.message);
    const [prevMessageValue, setPrevMessageValue] = useState(message.message);
    const [showIcons, setShowIcons] = useState(false);
    const [isEdit, setIsEdit] = useState(false);

    const [messageApi, contextHolder] = antdMessage.useMessage();

    const handleEdit = () => {
        setIsEdit(true);
    }

    const handleDelete = () => {
        const requestBody = {
            message: messageValue,
            line: message.line
        }
        requestToIde("messages/delete", requestBody, messageApi.error)
            .then((data) => {
                setMessageValue("");
                setPrevMessageValue("");
                console.log("deleteMessage got data : " + data);
            }).catch((error) => {
            console.log("deleteMessage got error : " + error);
        });
        props.reload();
    }

    const handleOKClick = () => {
        setIsEdit(false);
        const requestBody = {
            message: messageValue,
            line: message.line
        }
        requestToIde("messages/upsert", requestBody, messageApi.error)
            .then((data) => {
                props.reload();
                // setPrevMessageValue(messageValue);
                // setMessageValue(prevMessageValue);
                console.log("updateMessage got data : " + data);
            }).catch((error) => {
            console.log("updateMessage got error : " + error);
        });
    }

    const handleCancelClick = () => {
        setIsEdit(false);
        setMessageValue(prevMessageValue);
    }

    return (
        <>
            {contextHolder}
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
                        placeholder={"Add a new note !"}
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
                            <Popconfirm
                                overlayInnerStyle={{backgroundColor: theme.editorBackground}}
                                placement="left"
                                title={<span style={{ color: theme.text }}>Delete the note</span>}
                                description={<span style={{ color: theme.text }}>Are you sure to delete this note?</span>}
                                onConfirm={() => handleDelete()}
                                okText={<span style={{ color: theme.text }}>Yes</span>}
                                cancelButtonProps={{style: {backgroundColor: theme.background}}}
                                cancelText={<span style={{ color: theme.text }}>No</span>}
                            >
                                <Tooltip title="Delete">
                                    <Button
                                        size="small"
                                        shape="circle"
                                        icon={<DeleteOutlined/>}
                                    />
                                </Tooltip>
                            </Popconfirm>

                        </div>
                    )}
                </Space>
                {isEdit && (
                    <Flex gap="small" align="flex-end" style={{float: "right"}}>
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
                                backgroundColor: theme.background,
                                borderColor: theme.text
                            }}
                            onClick={() => handleCancelClick()}
                        >CANCEL</Button>
                    </Flex>
                )}
            </Space>
        </>

    );
}

export default Message

import {Button, Space, Tooltip} from "antd";
import {DeleteOutlined, EditOutlined} from "@ant-design/icons";
import TextareaAutosize from 'react-textarea-autosize';
import {useState} from "react";

const Message = () => {
    const [showIcons, setShowIcons] = useState(false);
    const [isEdit, setIsEdit] = useState(true);


    const handleEdit = () => {
        setIsEdit(true);
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
                    style={{resize: 'none', border: 'none'}}
                    defaultValue="I really enjoyed biking yesterday! And this is very very long string And this is very very long string And this is very very long string long may the king"
                    readOnly={true}
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
                <Space style={{float: "right"}}>
                    <Button type="primary">OK</Button>
                    <Button type="primary">CANCEL</Button>
                </Space>
            )}
        </Space>
    );
}

export default Message

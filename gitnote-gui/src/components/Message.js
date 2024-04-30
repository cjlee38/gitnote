import {Button, ConfigProvider, Flex, Space, Tooltip} from "antd";
import {DeleteOutlined, EditOutlined} from "@ant-design/icons";
import TextareaAutosize from 'react-textarea-autosize';
import { TinyColor } from '@ctrl/tinycolor';
import {useState} from "react";

const Message = (props) => {
    const [messageValue, setMessageValue] = useState(props.message);
    const [showIcons, setShowIcons] = useState(false);
    const [isEdit, setIsEdit] = useState(false);

    const colors1 = ['#6253E1', '#04BEFE'];
    const getHoverColors = (colors) =>
        colors.map((color) => new TinyColor(color).lighten(5).toString());
    const getActiveColors = (colors) =>
        colors.map((color) => new TinyColor(color).darken(5).toString());

    const handleEdit = () => {
        setIsEdit(true);
    }

    function handleOKClick() {
        setIsEdit(false);
    }

    function handleCancelClick() {
        setIsEdit(false);
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
                    <ConfigProvider
                        theme={{
                            components: {
                                Button: {
                                    colorPrimary: `linear-gradient(90deg,  ${colors1.join(', ')})`,
                                    colorPrimaryHover: `linear-gradient(90deg, ${getHoverColors(colors1).join(', ')})`,
                                    colorPrimaryActive: `linear-gradient(90deg, ${getActiveColors(colors1).join(', ')})`,
                                    lineWidth: 0,
                                },
                            },
                        }}
                    >
                        <Button type="primary" onClick={() => handleOKClick()}>OK</Button>
                        <Button type="primary" onClick={() => handleCancelClick()}>CANCEL</Button>
                    </ConfigProvider>

                </Flex>
            )}
        </Space>
    );
}

export default Message

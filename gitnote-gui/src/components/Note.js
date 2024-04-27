import {Button, List, Space, Tooltip} from "antd";

import Message from "./Message";

const Note = () => {
    const sample1 = "mock message 1";
    const sample2 = "mock message 2";
    const sample3 = "mock message 3";

    const data = [sample1, sample2, sample3]

    return (
        <div>
            <Message/>
            <Message/>
            <Message/>

            {/*<Space.Compact wrap>*/}
            {/*    <TextArea*/}
            {/*        style={{resize: "none"}}*/}
            {/*        defaultValue="I really enjoyed biking yesterday!"*/}
            {/*        readOnly={true}*/}
            {/*        rows={1}*/}
            {/*        cols={50}*/}
            {/*    >*/}
            {/*    </TextArea>*/}
            {/*    <Tooltip title="edit">*/}
            {/*        <Button*/}
            {/*            size="small"*/}
            {/*            shape="circle"*/}
            {/*            icon={<EditOutlined/>}*/}
            {/*        />*/}
            {/*    </Tooltip>*/}
            {/*    <Tooltip title="delete">*/}
            {/*        <Button*/}
            {/*            size="small"*/}
            {/*            shape="circle"*/}
            {/*            icon={<DeleteOutlined/>}*/}
            {/*        />*/}
            {/*    </Tooltip>*/}
            {/*</Space.Compact>*/}
            <List
                size="large"
                bordered
                dataSource={data}
                renderItem={(item) => <List.Item>{item}</List.Item>}
            />
        </div>
    );
}

export default Note;

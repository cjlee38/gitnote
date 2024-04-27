import {Divider, List} from "antd";

const Note = () => {
    const sample1 = "mock message 1";
    const sample2 = "mock message 2";
    const sample3 = "mock message 3";

    const data = [sample1, sample2, sample3]
    return (
        <div>
            <p>
                "test"
            </p>
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

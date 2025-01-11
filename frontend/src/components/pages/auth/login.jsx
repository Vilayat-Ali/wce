import React, { useState } from "react";
import { Form, Input, Button, Typography, Spin } from "antd";
import { Link, useNavigate } from "react-router-dom";

const { Title, Text } = Typography;

const Login = () => {
    const [isLoading, setLoading] = useState(false);
    const navigate = useNavigate();

    const handleLogin = (values) => {
        setLoading(true);
        setTimeout(() => {
            console.log("Form Values:", values);
            setLoading(false);
            alert("Login Successful!");
            navigate("/");
        }, 1000);
    };

    return (
        <div style={{ display: "flex", height: "100vh", justifyContent: "center", alignItems: "center" }}>
            <div style={{ width: "100%", maxWidth: "400px", padding: "24px", boxShadow: "0 4px 8px rgba(0, 0, 0, 0.1)", borderRadius: "8px" }}>
                {isLoading && (
                    <div style={{ display: "flex", justifyContent: "center", marginBottom: "16px" }}>
                        <Spin size="large" />
                    </div>
                )}

                <Title level={3} style={{ textAlign: "center", marginBottom: "16px" }}>
                    LOG IN
                </Title>

                <Form
                    layout="vertical"
                    onFinish={handleLogin}
                    initialValues={{ email: "", password: "" }}
                >
                    <Form.Item
                        label="Email ID"
                        name="email"
                        rules={[{ required: true, message: "Please enter your email!" }]}
                    >
                        <Input type="email" placeholder="Enter your email" />
                    </Form.Item>

                    <Form.Item
                        label="Password"
                        name="password"
                        rules={[{ required: true, message: "Please enter your password!" }]}
                    >
                        <Input.Password placeholder="Enter your password" />
                    </Form.Item>

                    <div style={{ display: "flex", justifyContent: "flex-end", marginBottom: "16px" }}>
                        <Link to="/forgot-password" style={{ fontWeight: "bold", color: "#673995" }}>
                            Forgot Password?
                        </Link>
                    </div>

                    <Form.Item>
                        <Button type="primary" htmlType="submit" block loading={isLoading}>
                            Log In
                        </Button>
                    </Form.Item>
                </Form>

                <Text style={{ display: "block", textAlign: "center", marginTop: "16px" }}>
                    Don&apos;t have an account?{" "}
                    <Link to="/signup" style={{ fontWeight: "bold", color: "#673995" }}>
                        Register
                    </Link>
                </Text>
            </div>
        </div>
    );
};

export default Login;

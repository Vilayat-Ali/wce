import React, { useState } from "react";
import { Form, Input, Button, Checkbox, Card, Typography, Row, Col, Modal, Spin, message } from "antd";
import { EyeOutlined, EyeInvisibleOutlined } from "@ant-design/icons";
import { useNavigate, Link } from "react-router-dom";

const { Title, Text } = Typography;

const Register = () => {
    const [formValues, setFormValues] = useState({
        fullName: "",
        email: "",
        mobileNumber: "",
        password: "",
        confirmPassword: "",
        companyName: "",
        agreeTerms: false,
    });
    const [isModalOpen, setIsModalOpen] = useState(false);
    const [isLoading, setLoading] = useState(false);
    const navigate = useNavigate();

    const handleAgreeTerms = () => {
        setFormValues((prevState) => ({
            ...prevState,
            agreeTerms: true,
        }));
        setIsModalOpen(false);
    };

    const handleSubmit = async (values) => {
        setLoading(true);
        try {
            // API endpoint for registration
            const apiEndpoint = "https://example.com/api/register"; 

            const response = await fetch(apiEndpoint, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify(values), // Send form values as JSON
            });

            const result = await response.json();
            if (response.ok) {
                message.success("Registration Successful!");
                navigate("/login");
            } else {
                message.error(result.message || "Registration failed!");
            }
        } catch (error) {
            message.error("An error occurred during registration!");
            console.error("Error:", error);
        } finally {
            setLoading(false);
        }
    };

    return (
        <div style={{ display: "flex", justifyContent: "center", alignItems: "center", minHeight: "100vh" }}>
            <Card style={{ width: "100%", maxWidth: "700px", padding: "24px" }}>
                {isLoading && (
                    <div
                        style={{
                            position: "absolute",
                            top: 0,
                            left: 0,
                            right: 0,
                            bottom: 0,
                            backgroundColor: "rgba(255, 255, 255, 0.7)",
                            display: "flex",
                            justifyContent: "center",
                            alignItems: "center",
                        }}
                    >
                        <Spin size="large" />
                    </div>
                )}

                <Title level={2} style={{ textAlign: "center", marginBottom: "24px" }}>
                    Join Us and Create Your Account
                </Title>

                <Form onFinish={handleSubmit} layout="vertical">
                    <Row gutter={24}>
                        <Col span={12}>
                            <Form.Item
                                label="Name"
                                name="fullName"
                                rules={[{ required: true, message: "Please input your full name!" }]}
                            >
                                <Input placeholder="Enter your full name" />
                            </Form.Item>
                        </Col>

                        <Col span={12}>
                            <Form.Item
                                label="Email"
                                name="email"
                                rules={[{ required: true, message: "Please enter your email!" }]}
                            >
                                <Input type="email" placeholder="Enter your email" />
                            </Form.Item>
                        </Col>
                    </Row>

                    <Row gutter={24}>
                        <Col span={12}>
                            <Form.Item
                                label="Mobile Number"
                                name="mobileNumber"
                                rules={[{ required: true, message: "Please enter your phone number" }]}
                            >
                                <Input type="number" placeholder="Enter your phone number" />
                            </Form.Item>
                        </Col>

                        <Col span={12}>
                            <Form.Item
                                label="Password"
                                name="password"
                                rules={[{ required: true, message: "Please enter your password!" }]}
                            >
                                <Input.Password
                                    placeholder="Enter your password"
                                    iconRender={(visible) => (visible ? <EyeInvisibleOutlined /> : <EyeOutlined />)}
                                />
                            </Form.Item>
                        </Col>
                    </Row>

                    <Row gutter={24}>
                        <Col span={12}>
                            <Form.Item label="College / Organisation Name" name="companyName">
                                <Input placeholder="Enter your College / Organisation name" />
                            </Form.Item>
                        </Col>

                        <Col span={12}>
                            <Form.Item
                                label="Confirm Password"
                                name="confirmPassword"
                                rules={[
                                    { required: true, message: "Please confirm your password!" },
                                    ({ getFieldValue }) => ({
                                        validator(_, value) {
                                            if (!value || getFieldValue("password") === value) {
                                                return Promise.resolve();
                                            }
                                            return Promise.reject(new Error("Passwords do not match!"));
                                        },
                                    }),
                                ]}
                            >
                                <Input.Password
                                    placeholder="Confirm your password"
                                    iconRender={(visible) => (visible ? <EyeInvisibleOutlined /> : <EyeOutlined />)}
                                />
                            </Form.Item>
                        </Col>
                    </Row>

                    <Form.Item
                        name="agreeTerms"
                        valuePropName="checked"
                        rules={[{ required: true, message: "You must agree to the terms and conditions!" }]}
                        style={{ display: "flex", justifyContent: "center" }}
                    >
                        <Checkbox
                            name="agreeTerms"
                            checked={formValues.agreeTerms}
                            onChange={(e) => setFormValues({ ...formValues, agreeTerms: e.target.checked })}
                            disabled={!formValues.agreeTerms && !isModalOpen}
                        >
                            I agree to the{" "}
                            <Text
                                as="span"
                                style={{
                                    color: "#673995",
                                    fontWeight: "bold",
                                    cursor: "pointer",
                                    textDecoration: "underline",
                                }}
                                onClick={() => setIsModalOpen(true)}
                            >
                                terms and conditions
                            </Text>
                        </Checkbox>
                    </Form.Item>

                    <Form.Item>
                        <Button type="primary" htmlType="submit" block loading={isLoading}>
                            Submit
                        </Button>
                    </Form.Item>

                    <div style={{ textAlign: "center" }}>
                        <Text>
                            Already have an account?{" "}
                            <Link to="/login" style={{ fontWeight: "bold", color: "#673995" }}>
                                Log in here
                            </Link>
                        </Text>
                    </div>
                </Form>

                <Modal
                    title="Terms and Conditions"
                    visible={isModalOpen}
                    onCancel={() => setIsModalOpen(false)}
                    footer={[
                        <Button key="back" type="primary" onClick={handleAgreeTerms}>
                            Agree
                        </Button>,
                    ]}
                >
                    <div>
                        <p>
                            By registering, you agree to our terms of service and privacy policy. Please read them carefully
                            before proceeding with registration.
                        </p>
                        <ul style={{ paddingLeft: "20px", margin: 0 }}>
                            <li style={{ marginBottom: "10px" }}>
                                All users must be at least 18 years old.
                            </li>
                            <li style={{ marginBottom: "10px" }}>
                                We may update our terms and conditions from time to time.
                            </li>
                            <li>Your account is personal and should not be shared.</li>
                        </ul>
                    </div>
                </Modal>
            </Card>
        </div>
    );
};

export default Register;

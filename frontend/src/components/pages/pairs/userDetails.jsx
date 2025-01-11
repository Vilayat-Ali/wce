// ProfilePage.jsx

import React from "react";
import { Card, Avatar, Typography, Row,Tag, Col, Divider } from "antd";
import { TrophyOutlined, MailOutlined, PhoneOutlined, HomeOutlined, UserOutlined } from "@ant-design/icons";
import image from "../../../../public/image.png";
const { Title, Text } = Typography;

import AppBase from '../../base/AppBase'

// Dummy data 
const generateDummyProfileData = () => ({
    username: "ahmad8929",
    name: "Mohammad Ahmad",
    skills: ["HTML","CSS","React", "JavaScript", "Ant Design", "Node.js"],
    address: "Madina Colony Muzaffarnagar, Uttar Pradesh, India",
    email: "m.ahmad8929@gmail.com",
    phone: "+91 8929691406",
    profileImage: image,
    trophies: 8,
});

const getRandomColor = () => {
    const colors = ["#673995", "#52c41a", "#1890ff", "#ff4d4f", "#faad14"];
    return colors[Math.floor(Math.random() * colors.length)];
  };

const ProfilePage = () => {
    const profileData = generateDummyProfileData();

    return (
            <div style={{ backgroundColor: "white",  padding: "20px" }}>
                    {/* Header Section */}
                    <Row align="middle" justify="space-between">
                        <Col>
                            <Row align="middle" gutter={16}>
                                <Col>
                                    <Avatar size={100} src={profileData.profileImage} />
                                </Col>
                                <Col>
                                    <Title level={2}>{profileData.name}</Title>
                                    <Text type="secondary">@{profileData.username}</Text>
                                </Col>
                            </Row>
                        </Col>
                        <Col style={{ textAlign: "right" }}>
                            <TrophyOutlined style={{ fontSize: "24px", color: "#673995" }} />
                            <Text style={{ fontSize: "18px", marginLeft: "8px" }}>{profileData.trophies}</Text>
                        </Col>
                    </Row>

                    <Divider />

                    {/* Details Section */}
                    <Row gutter={[16, 16]}>
                        <Col span={12}>
                            <Row align="middle" gutter={8}>
                                <Col>
                                    <UserOutlined style={{ fontSize: "18px", color: "#673995" }} />
                                </Col>
                                <Col>
                                    <Text strong>Username:</Text>
                                    <Text style={{ marginLeft: "8px" }}>{profileData.username}</Text>
                                </Col>
                            </Row>
                        </Col>

                        <Col span={12}>
                            <Row align="middle" gutter={8}>
                                <Col>
                                    <MailOutlined style={{ fontSize: "18px", color: "#673995" }} />
                                </Col>
                                <Col>
                                    <Text strong>Email:</Text>
                                    <Text style={{ marginLeft: "8px" }}>{profileData.email}</Text>
                                </Col>
                            </Row>
                        </Col>

                        <Col span={12}>
                            <Row align="middle" gutter={8}>
                                <Col>
                                    <PhoneOutlined style={{ fontSize: "18px", color: "#673995" }} />
                                </Col>
                                <Col>
                                    <Text strong>Phone:</Text>
                                    <Text style={{ marginLeft: "8px" }}>{profileData.phone}</Text>
                                </Col>
                            </Row>
                        </Col>

                        <Col span={12}>
                            <Row align="middle" gutter={8}>
                                <Col>
                                    <HomeOutlined style={{ fontSize: "18px", color: "#673995", marginRight:4 }} />
                                    <Text strong>Address:</Text>
                                    <Text style={{ marginLeft: "8px" }}>{profileData.address}</Text>
                                </Col>
                            </Row>
                        </Col>
                    </Row>

                    <Divider />

                    {/* Skills Section */}
                    <div>
          <Title level={4}>Skills</Title>
          <Row gutter={[8, 8]}>
            {profileData.skills.map((skill, index) => (
              <Col key={index}>
                <Tag color={getRandomColor()}>#{skill}</Tag>
              </Col>
            ))}
          </Row>
        </div>
            </div>
    );
};

export default ProfilePage;

import React, { useState } from "react";
import { Tabs, Typography, Divider, Row, Col } from "antd";
import TopPlayers from "./TopPlayers";
import AllPlayers from "./AllPlayers";
import AppBase from "../../base/AppBase";

const { TabPane } = Tabs;
const { Title, Text } = Typography;

const Leaderboard = () => {
  return (
    <AppBase>
      {/* Cool Header Section */}
      <div
        style={{
          backgroundColor: "#673995",
          padding: "30px 20px",
          color: "white",
          borderRadius: "8px",
        }}
      >
        <Row align="middle" justify="center">
          <Col style={{ textAlign: "center" }}>
            <Title level={2} style={{ margin: 0, color: "#fff" }}>
              Welcome to the Leaderboard
            </Title>
            <Text style={{ color: "#f5f5f5", fontSize: "16px" }}>
              Explore the top players and track your ranking among the best!
            </Text>
          </Col>
        </Row>
      </div>

      {/* Separator */}
      <Divider style={{ margin: "5px 0" }} />

      {/* Tabs Section */}
      <Tabs defaultActiveKey="1" centered>
        <TabPane tab="Leaderboard" key="1">
          <TopPlayers />
        </TabPane>
        <TabPane tab="All Players" key="2">
          <AllPlayers />
        </TabPane>
      </Tabs>
    </AppBase>
  );
};

export default Leaderboard;

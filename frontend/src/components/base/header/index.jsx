import React, { useState } from "react";
import { Layout, Button, Avatar, Dropdown,Menu, Space } from "antd";
import { MenuOutlined, CloseOutlined } from "@ant-design/icons";
import { useNavigate } from "react-router-dom";
const { Header } = Layout;

const Navbar = ({ onToggleDrawer, drawerVisible }) => {
  const [isLoggedIn, setIsLoggedIn] = useState(false);
  const [userName, setUserName] = useState("Ahmad");
  const navigate = useNavigate();
  const userMenu = (
    <Menu
      items={[
        { label: <a href="/profile">Profile</a>, key: "profile" },
        { label: <a href="/settings">Settings</a>, key: "settings" },
        { label: <Button type="text" onClick={() => setIsLoggedIn(false)}>Logout</Button>, key: "logout" },
      ]}
    />
  );

  return (
    <Header style={{ backgroundColor: "#001529", padding: "0 20px", display: "flex", alignItems: "center" }}>
      <Button
        icon={drawerVisible ? <CloseOutlined /> : <MenuOutlined />}
        type="text"
        style={{ color: "#fff", fontSize: "24px" }}
        onClick={onToggleDrawer}
      />

      <div style={{ marginLeft: "auto" }}>
        {!isLoggedIn ? (
          <Button type="primary"  onClick={() => navigate("/login")}>
            Login
          </Button>
        ) : (
          <Dropdown overlay={userMenu} placement="bottomRight" arrow>
            <Space>
              <Avatar style={{ backgroundColor: "#673995", verticalAlign: "middle" }}>
                {userName.charAt(0).toUpperCase()}
              </Avatar>
            </Space>
          </Dropdown>
        )}
      </div>
    </Header>
  );
};

export default Navbar;
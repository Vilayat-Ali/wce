import React, { useState } from "react";
import { Layout, Button, Avatar, Dropdown, Space, Drawer, Menu } from "antd";
import { Link } from "react-router-dom";
import { MenuOutlined, CloseOutlined } from "@ant-design/icons";
import SiderComponent from "../sider";

const { Header } = Layout;

const Navbar = () => {
  const [isLoggedIn, setIsLoggedIn] = useState(false);
  const [userName, setUserName] = useState("Ahmad");
  const [drawerVisible, setDrawerVisible] = useState(false);

  const menuItems = [
    { label: <Link to="/">Home</Link>, key: "home" },
    { label: <Link to="/about">About</Link>, key: "about" },
    { label: <Link to="/features">Features</Link>, key: "features" },
    { label: <Link to="/contact">Contact</Link>, key: "contact" },
  ];

  const userMenu = (
    <Menu
      items={[
        { label: <Link to="/profile">Profile</Link>, key: "profile" },
        { label: <Link to="/settings">Settings</Link>, key: "settings" },
        { label: <Button type="text" onClick={() => setIsLoggedIn(false)}>Logout</Button>, key: "logout" },
      ]}
    />
  );

  const handleDrawerToggle = () => {
    setDrawerVisible(!drawerVisible);
  };

  return (
    <Layout>
      {/* Sider Component for Menu */}
      <SiderComponent drawerVisible={drawerVisible} onClose={handleDrawerToggle} />

      <Layout>
        <Header style={{ display: "flex", alignItems: "center", padding: "0 20px", backgroundColor: "#001529" }}>
          {/* Logo */}
          <div style={{ color: "#fff", fontSize: "24px", fontWeight: "bold", marginRight: "auto" }}>
            <Link to="/" style={{ color: "#673995", textDecoration: "none" }}>WCE</Link>
          </div>

          {/* Hamburger Menu Icon */}
          <div
  style={{
    position: "absolute",
    left: "220px",
  }}
>
          <Button
            icon={drawerVisible ? <CloseOutlined /> : <MenuOutlined />}
            type="text"
            style={{ color: "#fff", fontSize: "24px" }}
            onClick={handleDrawerToggle}
          />
</div>
          {/* Login Button or User Dropdown */}
          <div style={{ marginLeft: "auto" }}>
            {!isLoggedIn ? (
              <Button type="primary" onClick={() => setIsLoggedIn(true)}>
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
      </Layout>
    </Layout>
  );
};

export default Navbar;

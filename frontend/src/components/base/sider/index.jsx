import React from "react";
import { Menu, Drawer } from "antd";
import { Link } from "react-router-dom";

const SiderComponent = ({ drawerVisible, onClose }) => {
  const menuItems = [
    { label: <Link to="/" style={{ color: "#fff", textDecoration: "none" }}>Home</Link>, key: "home" },
    { label: <Link to="/about" style={{ color: "#fff", textDecoration: "none" }}>About</Link>, key: "about" },
    { label: <Link to="/features" style={{ color: "#fff", textDecoration: "none" }}>Features</Link>, key: "features" },
    { label: <Link to="/contact" style={{ color: "#fff", textDecoration: "none" }}>Contact</Link>, key: "contact" },
  ];

  return (
    <Drawer
      title={
        <div style={{ color: "#fff", fontSize: "24px", fontWeight: "bold" }}>
          <Link to="/" style={{ color: "#fff", textDecoration: "none" }}>WCE</Link>
        </div>
      }
      placement="left"
      closable={false}
      onClose={onClose}
      open={drawerVisible}
      width={200}
      bodyStyle={{ backgroundColor: "#001529", padding: "0" }}
      headerStyle={{ backgroundColor: "#001529", borderBottom: "none" }}
    >
      <Menu
        mode="vertical"
        items={menuItems}
        style={{
          backgroundColor: "#001529",
          border: "none",
          color: "#fff",
        }}
      />
    </Drawer>
  );
};

export default SiderComponent;

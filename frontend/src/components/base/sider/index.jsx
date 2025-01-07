import React, { useState, useRef, useEffect, useMemo } from "react";
import { Divider, Menu, Tour } from "antd";
import {
  AppstoreOutlined,
  ContactsOutlined,
  SettingOutlined
} from "@ant-design/icons";
import { Link, useLocation } from "react-router-dom";
import logo from "../../../../public/logo.png";
import shortLogo from "../../../../public/shortLogo.png";

const AppSider = ({ collapsed, onCollapse }) => {
  const location = useLocation();
  const [tourVisible, setTourVisible] = useState(false);

  const ProfileRef = useRef(null);
  const PairsRef = useRef(null);
  const StatsRef = useRef(null);

  const selectedKeys = useMemo(
    () => [location?.pathname],
    [location?.pathname]
  );
  const openKeys = useMemo(
    () => [`/${location?.pathname.split("/")[1]}`],
    [location?.pathname]
  );

  const [selectedMenuKeys, setSelectedMenuKeys] = useState(selectedKeys);
  const [openMenuKeys, setOpenMenuKeys] = useState(openKeys);

  const steps = [
    {
      title: "Profile",
      description: "Here you can view your Profile.",
      target: () => ProfileRef.current,
      placement: "right",
    },
    {
      title: "Pairs",
      description: "Here you can manage your Pairs.",
      target: () => PairsRef.current,
      placement: "right",
    },
    {
      title: "Stats",
      description: "Here you can manage your Stats.",
      target: () => StatsRef.current,
      placement: "right",
    },
  ];

  useEffect(() => {
    setSelectedMenuKeys([location?.pathname]);
    setOpenMenuKeys([`/${location?.pathname.split("/")[1]}`]);
  }, [location]);

  return (
    <div style={{ height: "100vh", backgroundColor: "#001529", color: "#fff" }}>
      <div
        style={{
          height: 64,
          display: "flex",
          justifyContent: "center",
          alignItems: "center",
        }}
      >
        <img alt="WCE" style={{ height: "100%", width: "auto" }} src={collapsed ? shortLogo : logo} />
      </div>
      <Divider style={{ margin: 0, backgroundColor: "#fff" }} />

      <Menu
        style={{ height: "calc(100vh - 64px)", borderRight: 0, backgroundColor: "#001529" }}
        mode="inline"
        selectedKeys={selectedMenuKeys}
        openKeys={openMenuKeys}
        onOpenChange={(keys) => setOpenMenuKeys(keys)}
        theme="dark"
      >
        <Menu.Item key="/Profile" icon={<AppstoreOutlined />}>
          <Link to="/" ref={ProfileRef} style={{ color: "#fff" }}>Profile</Link>
        </Menu.Item>
        <Menu.Item key="/pairs" icon={<ContactsOutlined />}>
          <Link to="/pairs" ref={PairsRef} style={{ color: "#fff" }}>Pairs</Link>
        </Menu.Item>
        <Menu.Item key="/Stats" icon={<SettingOutlined />}>
          <Link to="/stats" ref={StatsRef} style={{ color: "#fff" }}>Stats</Link>
        </Menu.Item>
      </Menu>
      {tourVisible && (
        <Tour
          steps={steps}
          onFinish={() => setTourVisible(false)}
          onCancel={() => setTourVisible(false)}
        />
      )}
    </div>
  );
};

export default AppSider;
import React, { useState } from "react";
import { Table, Avatar, Space, Modal } from "antd";
import UserDetail from "./userDetails";

const TopPlayers = () => {
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [selectedPlayer, setSelectedPlayer] = useState(null);

  const leaderboardData = Array.from({ length: 5 }, (_, index) => ({
    key: index + 1,
    sno: index + 1,
    name: `Player ${index + 1}`,
    rank: Math.ceil(Math.random() * 10),
    trophies: Math.ceil(Math.random() * 10),
  }));

  const handleAvatarClick = (player) => {
    setSelectedPlayer(player);
    setIsModalOpen(true);
  };

  const handleModalClose = () => {
    setIsModalOpen(false);
    setSelectedPlayer(null);
  };

  const columns = [
    {
      title: "S.No.",
      dataIndex: "sno",
      key: "sno",
    },
    {
      title: "Name",
      dataIndex: "name",
      key: "name",
      render: (name, record) => (
        <Space>
          <Avatar
            style={{ backgroundColor: "#673995", color: "#fff", cursor: "pointer" }}
            onClick={() => handleAvatarClick(record)}
          >
            {name.charAt(0).toUpperCase()}
          </Avatar>
          {name}
        </Space>
      ),
    },
    {
      title: "Rank",
      dataIndex: "rank",
      key: "rank",
    },
    {
      title: "Trophies",
      dataIndex: "trophies",
      key: "trophies",
    },
  ];

  return (
    <>
      <Table columns={columns} dataSource={leaderboardData} pagination={false} />
      <Modal
        title="User Details"
        visible={isModalOpen}
        onCancel={handleModalClose}
        footer={null}
        width={700}
        centered
      >
        {selectedPlayer && <UserDetail user={selectedPlayer} />}
      </Modal>
    </>
  );
};

export default TopPlayers;

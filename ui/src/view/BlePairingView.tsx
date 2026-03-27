import {
  DisconnectOutlined,
  KeyOutlined,
  LinkOutlined,
  MinusOutlined,
} from "@ant-design/icons";
import {
  BleDevice,
  BleDevicesList,
  BleDevicePairingState,
} from "@caniot-controller/caniot-api-grpc-web/api/ng_copro_pb";
import { Badge, Button, Descriptions, Space, TableProps, Table, Tag, Tooltip } from "antd";
import React, { useEffect, useState } from "react";
import coproStore from "../store/CoproStore";

interface IBlePairingViewProps {
  isMobile?: boolean;
  refreshInterval?: number;
}

interface BleDeviceRow {
  key: string;
  mac: string;
  connected: boolean;
  pairingState: BleDevicePairingState;
  pairingCode?: number;
  connectionEvents: number;
  pairingEvents: number;
  commandsReceived: number;
  lastSeen?: Date;
}

function BlePairingView({ isMobile = false, refreshInterval = 2000 }: IBlePairingViewProps) {
  const [rows, setRows] = useState<BleDeviceRow[]>([]);
  const [loading, setLoading] = useState(true);
  const [time, setTime] = useState(Date.now());
  const [removingBonds, setRemovingBonds] = useState(false);

  const handleRemoveBonds = () => {
    setRemovingBonds(true);
    coproStore.bleRemoveBonds(() => {
      setRemovingBonds(false);
      setTime(Date.now());
    });
  };

  useEffect(() => {
    setLoading(true);
    coproStore.getBleDevices((resp: BleDevicesList) => {
      setRows(
        resp.getDevicesList().map((d: BleDevice) => ({
          key: d.getMac(),
          mac: d.getMac(),
          connected: d.getConnected(),
          pairingState: d.getPairingState(),
          pairingCode: d.hasPairingCode() ? d.getPairingCode() : undefined,
          connectionEvents: d.getStats()?.getConnectionEvents() ?? 0,
          pairingEvents: d.getStats()?.getPairingEvents() ?? 0,
          commandsReceived: d.getStats()?.getCommandsReceived() ?? 0,
          lastSeen: d.getLastSeen()?.toDate(),
        }))
      );
      setLoading(false);
    });

    const interval = setInterval(() => setTime(Date.now()), refreshInterval);
    return () => clearInterval(interval);
  }, [time, refreshInterval]);

  const columns: TableProps<BleDeviceRow>["columns"] = [
    {
      title: "Adresse",
      dataIndex: "mac",
      key: "mac",
      width: 200,
      render: (mac) => <code>{mac}</code>,
    },
    {
      title: "Connexion",
      dataIndex: "connected",
      key: "connected",
      width: 160,
      render: (connected) => (
        <Badge
          status={connected ? "success" : "default"}
          text={connected ? "Connecté" : "Déconnecté"}
        />
      ),
    },
    {
      title: "Appairage",
      dataIndex: "pairingState",
      key: "pairingState",
      width: 260,
      render: (pairingState, record) => {
        switch (pairingState) {
          case BleDevicePairingState.BLE_PAIRING_SUCCEEDED:
            return (
              <Tag icon={<LinkOutlined />} color="blue">
                Appairé
              </Tag>
            );
          case BleDevicePairingState.BLE_PAIRING_FAILED:
            return (
              <Tag icon={<DisconnectOutlined />} color="red">
                Échec
              </Tag>
            );
          case BleDevicePairingState.BLE_PAIRING_PENDING:
            return (
              <Tooltip title="Entrer ce code sur l'appareil">
                <Tag icon={<KeyOutlined />} color="orange">
                  En cours — code : <strong>{record.pairingCode}</strong>
                </Tag>
              </Tooltip>
            );
          default:
            return (
              <Tag icon={<MinusOutlined />} color="default">
                Aucun
              </Tag>
            );
        }
      },
    },
    {
      title: "Connexions",
      dataIndex: "connectionEvents",
      key: "connectionEvents",
      width: 120,
      responsive: ["md"],
    },
    {
      title: "Appairages",
      dataIndex: "pairingEvents",
      key: "pairingEvents",
      width: 120,
      responsive: ["md"],
    },
    {
      title: "Commandes",
      dataIndex: "commandsReceived",
      key: "commandsReceived",
      width: 120,
      responsive: ["md"],
    },
    {
      title: "Dernière activité",
      dataIndex: "lastSeen",
      key: "lastSeen",
      width: 200,
      responsive: ["md"],
      render: (lastSeen?: Date) =>
        lastSeen ? lastSeen.toLocaleString() : "—",
    },
  ];

  const expandable: TableProps<BleDeviceRow>["expandable"] = isMobile
    ? {
        expandedRowRender: (record) => (
          <Descriptions size="small" column={2} style={{ margin: 0 }}>
            <Descriptions.Item label="Connexions">{record.connectionEvents}</Descriptions.Item>
            <Descriptions.Item label="Appairages">{record.pairingEvents}</Descriptions.Item>
            <Descriptions.Item label="Commandes">{record.commandsReceived}</Descriptions.Item>
            <Descriptions.Item label="Dernière activité">
              {record.lastSeen ? record.lastSeen.toLocaleString() : "—"}
            </Descriptions.Item>
          </Descriptions>
        ),
      }
    : undefined;

  return (
    <Space direction="vertical" style={{ width: "100%" }}>
      <Table
        dataSource={rows}
        columns={columns}
        expandable={expandable}
        loading={loading}
        size={isMobile ? "small" : "middle"}
        pagination={false}
        locale={{ emptyText: "Aucun appareil BLE détecté" }}
      />
      <Button
        type="primary"
        danger
        loading={removingBonds}
        onClick={handleRemoveBonds}
      >
        Supprimer tous les appairages
      </Button>
    </Space>
  );
}

export default BlePairingView;


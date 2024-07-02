import { Button, Col, List, Row } from "antd";
import React, { useEffect } from "react";
import LoadableCard from "../components/LoadableCard";
import ListLabelledItem from "../components/ListLabelledItem";
import emulationStore from "../store/EmulationStore";
import {
  EmuRequest,
  Req,
  Status,
} from "@caniot-controller/caniot-api-grpc-web/api/ng_emulation_pb";
import ListGridItem from "../components/ListGridItem";

interface IEmulationViewProps {
  isMobile?: boolean;
}

function EmulationView({ isMobile = false }: IEmulationViewProps) {
  const [emulationSupported, setEmulationSupported] = React.useState<boolean>(false);

  useEffect(() => {
    emulationStore.get((resp: Status) => {
      setEmulationSupported(resp.getFeatureEnabled());
    });
  }, []);

  const sendEmulationEvent = (event: EmuRequest) => {
    return () => {
      const req = new Req();
      req.setEvent(event);
      emulationStore.set(req, (resp: Status) => {
        // ignore result
      });
    };
  };

  return (
    <Row gutter={16}>
      <Col xl={14} xs={24} style={{ marginBottom: 16 }}>
        <LoadableCard title="Actions simulées" loading={false} status={emulationSupported}>
          <List>
            <List.Item>
              <span style={{ fontWeight: "bold" }}>Alarme</span>
            </List.Item>

            <ListGridItem
              label="Détecteur"
              description="Simule une présence au niveau des détecteurs"
              isMobile={isMobile}
            >
              <Button
                type="primary"
                onClick={sendEmulationEvent(EmuRequest.OUTDOOR_ALARM_PRESENCE)}
              >
                Simuler présence
              </Button>
            </ListGridItem>
            <ListGridItem
              label="Sabotage"
              description="Simule le sabotage des détecteurs"
              isMobile={isMobile}
            >
              <Button
                type="primary"
                onClick={sendEmulationEvent(EmuRequest.OUTDOOR_ALARM_SABOTAGE)}
              >
                Simuler sabotage
              </Button>
            </ListGridItem>
            <ListGridItem
              label="Retour à la normale"
              description="Simule le retour à la normale des détecteurs"
              isMobile={isMobile}
            >
              <Button type="primary" onClick={sendEmulationEvent(EmuRequest.OUTDOOR_ALARM_CLEAR)}>
                Retour à la normale
              </Button>
            </ListGridItem>
          </List>
        </LoadableCard>
      </Col>
    </Row>
  );
}

export default EmulationView;

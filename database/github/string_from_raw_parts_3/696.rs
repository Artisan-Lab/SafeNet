/*
From: https://github.com/yutiansut/datafuse/blob/6a37c9c009d253f4920b03dd9b0273f3acdc75b2/src/query/service/src/api/rpc/flight_actions.rs#L97
*/

impl TryInto<FlightAction> for Action {
    type Error = Status;

    fn try_into(self) -> Result<FlightAction, Self::Error> {
        match self.r#type.as_str() {
            "InitQueryFragmentsPlan" => {
                Ok(FlightAction::InitQueryFragmentsPlan(self.body.try_into()?))
            }
            "InitNodesChannel" => Ok(FlightAction::InitNodesChannel(self.body.try_into()?)),
            "ExecutePartialQuery" => unsafe {
                let (buf, length, capacity) = self.body.into_raw_parts();
                Ok(FlightAction::ExecutePartialQuery(String::from_raw_parts(
                    buf, length, capacity,
                )))
            },
            un_implemented => Err(Status::unimplemented(format!(
                "UnImplement action {}",
                un_implemented
            ))),
        }
    }
}
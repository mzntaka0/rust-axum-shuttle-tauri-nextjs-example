import React from "react";
import { Stack, Typography } from "@mui/material";

const Component: React.FC = () => {
  return (
    <Stack>
      <Typography sx={{ color: "primary.main" }}>Delete this todo?</Typography>
    </Stack>
  );
};

export default Component;

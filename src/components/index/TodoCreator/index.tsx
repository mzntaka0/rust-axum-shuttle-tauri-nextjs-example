import React from 'react'
import {Stack, Box, TextField, Typography, FormControl, Button, CircularProgress} from '@mui/material'
import { useForm, Controller, SubmitHandler } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";

import { Schema, FormType } from "./validators";

type Props = {
  defaultValues?: FormType;
};

const Component: React.FC<Props> = (props) => {
  const {
    defaultValues = {
      text: "",
    },
  } = props;
  const { control, handleSubmit, formState, reset } = useForm<FormType>({
    mode: "all",
    defaultValues,
    resolver: zodResolver(Schema),
  });

  const onSubmit: SubmitHandler<FormType> = async (values) => {
    setTimeout(() => {console.log(values)}, 100)
    reset()
  };
  return (
    <Stack width="100%" maxWidth="800px">
      <Box width="100%" component="form" onSubmit={handleSubmit(onSubmit)}>
        <Stack
          justifyContent="center"
          alignItems="flex-end"
          direction="row"
          width="100%"
          spacing={3}
        >
          <Stack width="100%" direction="row" spacing={2}>
            <FormControl sx={{ width: "100%" }}>
              <Typography variant="caption">Register a Todo</Typography>
              <Controller
                name="text"
                control={control}
                render={({ field }) => {
                  return (
                    <TextField
                      sx={{
                        bgcolor: "rgba(255, 255, 255, 0.2)",
                      }}
                      margin="none"
                      size="small"
                      {...field}
                    />
                  );
                }}
              />
              {formState.errors.text && (
                <Box
                  sx={{
                    color: "error.main",
                    fontSize: "14px",
                  }}
                  role="alert"
                >
                  {formState.errors.text?.message}
                </Box>
              )}
            </FormControl>
          </Stack>
          <Stack
            sx={{
              boxSizing: "border-box",
            }}
            justifyContent="flex-end"
            alignItems="center"
            spacing={1}
            direction="row"
          >
            <Button
              type="submit"
              variant="contained"
              disabled={formState.isSubmitting || !formState.isValid}
              //disabled={formState.isSubmitting}
              startIcon={
                formState.isSubmitting ? <CircularProgress size={15} /> : null
              }
            >
              Add
            </Button>
          </Stack>
          </Stack>
        </Box>
      </Stack>
  )
}

export default Component

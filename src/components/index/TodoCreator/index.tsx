import React from "react";
import {
  Stack,
  Box,
  TextField,
  Typography,
  FormControl,
  Button,
  CircularProgress,
} from "@mui/material";
import { useForm, Controller, SubmitHandler } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { useQueryClient, useMutation } from "react-query";
import aspida from "@aspida/axios";
import { useSnackbar } from "notistack";

import { api as axiosApi } from "lib/fetcher";
import api from "api/$api";
import { Schema, FormType } from "./validators";
import { createTodo } from "components/index/common/functions/createTodo";

type Props = {
  defaultValues?: FormType;
};

const Component: React.FC<Props> = (props) => {
  const {
    defaultValues = {
      text: "",
    },
  } = props;

  const client = api(aspida(axiosApi));
  const queryClient = useQueryClient();
  const { mutateAsync } = useMutation(createTodo);

  const { control, handleSubmit, formState, reset } = useForm<FormType>({
    mode: "all",
    defaultValues,
    resolver: zodResolver(Schema),
  });

  const { enqueueSnackbar } = useSnackbar();

  const onSubmit: SubmitHandler<FormType> = async (values) => {
    await mutateAsync(values, {
      onSuccess: async () => {
        await queryClient.invalidateQueries(client.todos.$path());
        enqueueSnackbar("Todo has been created", {
          variant: "success",
        });
      },
      onError: () => {
        enqueueSnackbar("Failed to create a todo", {
          variant: "error",
        });
      },
    });
    reset();
  };
  return (
    <Stack width="100%" maxWidth="800px">
      <Box width="100%" component="form" onSubmit={handleSubmit(onSubmit)}>
        <Stack
          justifyContent="center"
          alignItems="center"
          direction="row"
          width="100%"
          spacing={3}
        >
          <Stack width="100%" direction="row" spacing={2}>
            <FormControl sx={{ width: "100%" }}>
              <Typography variant="caption">Create a Todo</Typography>
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
              <Box
                sx={{
                  color: "error.main",
                  fontSize: "14px",
                  height: "21px",
                }}
                role="alert"
              >
                {formState.errors.text && formState.errors.text?.message}
              </Box>
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
  );
};

export default Component;

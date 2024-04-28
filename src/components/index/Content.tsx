"use client"
import React from 'react'
import {Stack} from '@mui/material'
import { Theme } from "@mui/material/styles";

import TodoCreator from './TodoCreator'

const Content: React.FC = () => {
  return (
    <Stack
      sx={{
        width: '100%',
        height: '100%',
        p: (theme: Theme) => theme.spacing(0, 3, 0, 3),
      }}
    >
      <Stack
      sx={{
        width: '100%',
        height: '100%',
        p: 3,
        alignItems: 'center'
      }}

      >
        <TodoCreator />
      </Stack>
    </Stack>
  )
}

export default Content

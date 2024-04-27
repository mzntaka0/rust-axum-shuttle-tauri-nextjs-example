"use client";
import { createTheme, Theme } from "@mui/material";

const theme: Theme = createTheme({
  palette: {
    primary: {
      main: "#718096",
      light: "#a0afc7",
      dark: "#455468",
      contrastText: "#EDF2F7",
    },
    secondary: {
      main: "#01579b",
      light: "#4f83cc",
      dark: "#002f6c",
      contrastText: "#FFFFFF",
    },
    background: {
      default: "#1a202c",
      paper: "#FFFFFF",
    },
    text: {
      primary: "#EDF2F7",
      secondary: "#CBD5E0",
      disabled: "#FFFFFF",
    },
    common: {
      white: "#EDF2F7",
    },
  },
  components: {
    MuiButton: {
      styleOverrides: {
        root: {
          textTransform: "none",
        },
      },
    },
  },
  typography: {
    fontFamily:
      '"ヒラギノ角ゴシック Pro", "Hiragino Kaku Gothic Pro","游ゴシック体", "Yu Gothic","メイリオ","Meiryo","Osaka","ＭＳ Ｐゴシック","MS PGothic","sans-serif"',
  },
});

export default theme;

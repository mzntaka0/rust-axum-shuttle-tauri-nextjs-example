"use client";
import { Inter } from "next/font/google";
import { Box } from "@mui/material";
import { ErrorOutline } from "@mui/icons-material";
import { AppRouterCacheProvider } from "@mui/material-nextjs/v14-appRouter";
import { ThemeProvider } from "@mui/material/styles";
import CssBaseline from "@mui/material/CssBaseline";
import { MutationCache, QueryClient, QueryClientProvider } from "react-query";
import { SnackbarProvider } from "notistack";

import theme from "theme";
import "styles/globals.css";

const inter = Inter({ subsets: ["latin"] });

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      //refetchOnWindowFocus: false,
      //refetchOnMount: true,
      //refetchInterval: 100,
      //refetchOnReconnect: true,
      //staleTime: 0,
      retry: (failureCount, err: any) =>
        err.response?.status !== 403 && failureCount < 3,
    },
    mutations: {
      retry: (failureCount, err: any) =>
        err.response?.status !== 400 && failureCount < 1,
    },
  },
  mutationCache: new MutationCache({
    onError: (err) => {
      console.error(err);
    },
  }),
});

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className={inter.className} suppressHydrationWarning={true}>
        <AppRouterCacheProvider options={{ enableCssLayer: true }}>
          <ThemeProvider theme={theme}>
            <CssBaseline />
            <QueryClientProvider client={queryClient}>
              <SnackbarProvider
                anchorOrigin={{
                  vertical: "top",
                  horizontal: "center",
                }}
                autoHideDuration={3000}
                iconVariant={{
                  error: (
                    <Box mr={1} display="flex" alignItems="center">
                      <ErrorOutline style={{ width: "20px", height: "20px" }} />
                    </Box>
                  ),
                }}
              >
                {children}
              </SnackbarProvider>
            </QueryClientProvider>
          </ThemeProvider>
        </AppRouterCacheProvider>
      </body>
    </html>
  );
}

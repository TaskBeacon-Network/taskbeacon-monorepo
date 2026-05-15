import type { Metadata } from "next";

export const metadata: Metadata = {
  title: "TaskBeacon",
  description: "Decentralized task management on Stellar/Soroban",
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}

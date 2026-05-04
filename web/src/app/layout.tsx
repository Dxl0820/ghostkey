import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "GhostKey — Secrets Manager",
  description: "Local-first secrets manager for developers. Securely store and manage your API keys, database credentials, and environment variables.",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en" className="scroll-smooth">
      <body className="min-h-screen bg-surface-secondary text-text-primary font-sans antialiased">
        {children}
      </body>
    </html>
  );
}

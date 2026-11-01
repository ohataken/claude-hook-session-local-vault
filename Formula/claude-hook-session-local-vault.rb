class ClaudeHookSessionLocalVault < Formula
  desc "My little utility for Claude Code hooks"
  homepage "https://github.com/ohataken/claude-hook-session-local-vault"
  version "0.0.1"

  on_arm do
    url "https://github.com/ohataken/claude-hook-session-local-vault/releases/download/v#{version}/claude-hook-session-local-vault-aarch64-apple-darwin.tar.gz"
    sha256 "0000000000000000000000000000000000000000000000000000000000000000"
  end

  def install
    bin.install "claude-hook-session-local-vault"
  end
end

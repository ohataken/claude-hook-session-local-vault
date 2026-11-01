class ClaudeHookSessionLocalVault < Formula
  desc "My little utility for Claude Code hooks"
  homepage "https://github.com/ohataken/claude-hook-session-local-vault"
  version "0.0.1"

  on_arm do
    url "https://github.com/ohataken/claude-hook-session-local-vault/releases/download/v#{version}/claude-hook-session-local-vault-aarch64-apple-darwin.tar.gz"
    sha256 "4664ec6163e66b68cee450a2124d84b3c8aad074765f68419e0f22310a89aaa9"
  end

  def install
    bin.install "claude-hook-session-local-vault"
  end
end

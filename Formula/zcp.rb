# typed: false
# frozen_string_literal: true

# Homebrew formula for ZCP (ZeroCopy Auditor)
# Install: brew install zerocopy-systems/tap/zcp
class Zcp < Formula
  desc "HFT infrastructure latency audit tool — quantify your signing Jitter Tax"
  homepage "https://zerocopy.systems"
  version "1.0.0"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/zerocopy-systems/zcp/releases/download/v#{version}/zcp-macos-arm64"
      sha256 "PLACEHOLDER_SHA256_MACOS_ARM64"
    end
    on_intel do
      url "https://github.com/zerocopy-systems/zcp/releases/download/v#{version}/zcp-macos-x86_64"
      sha256 "PLACEHOLDER_SHA256_MACOS_X86_64"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/zerocopy-systems/zcp/releases/download/v#{version}/zcp-linux-arm64"
      sha256 "PLACEHOLDER_SHA256_LINUX_ARM64"
    end
    on_intel do
      url "https://github.com/zerocopy-systems/zcp/releases/download/v#{version}/zcp-linux-x86_64"
      sha256 "PLACEHOLDER_SHA256_LINUX_X86_64"
    end
  end

  def install
    binary = Dir["zcp-*"].first || "zcp"
    bin.install binary => "zcp"
  end

  test do
    assert_match "zerocopy-audit", shell_output("#{bin}/zcp --version")
  end
end

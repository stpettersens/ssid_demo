task :default => [:cleanlock] do
  sh "cargo build --release"
  sh "target/release/ssid_demo"
end

task :cleanlock do
  #File.delete("Cargo.lock")
end

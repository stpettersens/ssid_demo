task :default  do
  sh "cargo build --release"
end

task :test do
  sh "target/release/ssid_demo"
end

task :cleanlock do
  File.delete("Cargo.lock")
end

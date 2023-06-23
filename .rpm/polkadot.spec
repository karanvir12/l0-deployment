%define debug_package %{nil}

Name: peer
Summary: Implementation of a https://peer.network node in Rust based on the Substrate framework.
Version: @@VERSION@@
Release: @@RELEASE@@%{?dist}
License: GPLv3
Group: Applications/System
Source0: %{name}-%{version}.tar.gz

Requires: systemd, shadow-utils
Requires(post): systemd
Requires(preun): systemd
Requires(postun): systemd

BuildRoot: %{_tmppath}/%{name}-%{version}-%{release}-root

%description
%{summary}


%prep
%setup -q


%install
rm -rf %{buildroot}
mkdir -p %{buildroot}
cp -a * %{buildroot}

%post
config_file="/etc/default/peer"
getent group peer >/dev/null || groupadd -r peer
getent passwd peer >/dev/null || \
    useradd -r -g peer -d /home/peer -m -s /sbin/nologin \
    -c "User account for running peer as a service" peer
if [ ! -e "$config_file" ]; then
    echo 'peer_CLI_ARGS=""' > /etc/default/peer
fi
exit 0

%clean
rm -rf %{buildroot}

%files
%defattr(-,root,root,-)
%{_bindir}/*
/usr/lib/systemd/system/peer.service

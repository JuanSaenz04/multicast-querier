include $(TOPDIR)/rules.mk

PKG_NAME:=multicast-querier
PKG_VERSION:=0.1.0
PKG_RELEASE:=1

PKG_MAINTAINER:=Juani
PKG_LICENSE:=MIT

include $(INCLUDE_DIR)/package.mk
include $(TOPDIR)/feeds/packages/lang/rust/rust-package.mk

define Package/multicast-querier
  SECTION:=net
  CATEGORY:=Network
  TITLE:=Multicast Querier Service
  DEPENDS:=$(RUST_ARCH_DEPENDS)
endef

define Package/multicast-querier/description
  A lightweight IGMPv3/MLDv2 multicast querier written in Rust.
endef

define Package/multicast-querier/install
	$(INSTALL_DIR) $(1)/usr/bin
	$(INSTALL_BIN) $(PKG_INSTALL_DIR)/bin/multicast-querier $(1)/usr/bin/
	
	$(INSTALL_DIR) $(1)/etc/init.d
	$(INSTALL_BIN) ./files/multicast-querier.init $(1)/etc/init.d/multicast-querier
endef

$(eval $(call BuildPackage,multicast-querier))

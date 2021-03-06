mod address;
pub use self::address::Address;
mod auth_config;
pub use self::auth_config::AuthConfig;
mod body;
pub use self::body::Body;
mod body_1;
pub use self::body_1::Body1;
mod body_2;
pub use self::body_2::Body2;
mod body_3;
pub use self::body_3::Body3;
mod build_info;
pub use self::build_info::BuildInfo;
mod cluster_info;
pub use self::cluster_info::ClusterInfo;
mod commit;
pub use self::commit::Commit;
mod config;
pub use self::config::Config;
mod config_spec;
pub use self::config_spec::ConfigSpec;
mod container;
pub use self::container::Container;
mod container_1;
pub use self::container_1::Container1;
mod container_config;
pub use self::container_config::ContainerConfig;
mod container_create_body_networking_config;
pub use self::container_create_body_networking_config::ContainerCreateBodyNetworkingConfig;
mod container_summary;
pub use self::container_summary::{
    ContainerHostConfig, ContainerNetworkSettings, ContainerSummary,
};
mod container_summary_inner;
pub use self::container_summary_inner::ContainerSummaryInner;
mod container_summary_inner_host_config;
pub use self::container_summary_inner_host_config::ContainerSummaryInnerHostConfig;
mod container_summary_inner_network_settings;
pub use self::container_summary_inner_network_settings::ContainerSummaryInnerNetworkSettings;
mod create_image_info;
pub use self::create_image_info::CreateImageInfo;
mod device_mapping;
pub use self::device_mapping::DeviceMapping;
mod driver;
pub use self::driver::Driver;
mod endpoint_ipam_config;
pub use self::endpoint_ipam_config::EndpointIpamConfig;
mod endpoint_port_config;
pub use self::endpoint_port_config::EndpointPortConfig;
mod endpoint_settings;
pub use self::endpoint_settings::EndpointSettings;
mod endpoint_spec;
pub use self::endpoint_spec::EndpointSpec;
mod engine_description;
pub use self::engine_description::EngineDescription;
mod engine_description_plugins;
pub use self::engine_description_plugins::EngineDescriptionPlugins;
mod error_detail;
pub use self::error_detail::ErrorDetail;
mod error_response;
pub use self::error_response::ErrorResponse;
mod exec_config;
pub use self::exec_config::ExecConfig;
mod exec_start_config;
pub use self::exec_start_config::ExecStartConfig;
mod generic_resources;
pub use self::generic_resources::GenericResources;
mod generic_resources_inner;
pub use self::generic_resources_inner::GenericResourcesInner;
mod generic_resources_inner_discrete_resource_spec;
pub use self::generic_resources_inner_discrete_resource_spec::GenericResourcesInnerDiscreteResourceSpec;
mod generic_resources_inner_named_resource_spec;
pub use self::generic_resources_inner_named_resource_spec::GenericResourcesInnerNamedResourceSpec;
mod graph_driver_data;
pub use self::graph_driver_data::GraphDriverData;
mod health_config;
pub use self::health_config::HealthConfig;
mod host_config_log_config;
pub use self::host_config_log_config::HostConfigLogConfig;
mod host_config_port_bindings;
pub use self::host_config_port_bindings::HostConfigPortBindings;
mod id_response;
pub use self::id_response::IdResponse;
mod image;
pub use self::image::Image;
mod image_delete_response_item;
pub use self::image_delete_response_item::ImageDeleteResponseItem;
mod image_metadata;
pub use self::image_metadata::ImageMetadata;
mod image_root_fs;
pub use self::image_root_fs::ImageRootFs;
mod image_summary;
pub use self::image_summary::ImageSummary;
mod index_info;
pub use self::index_info::IndexInfo;
mod inline_response_200;
pub use self::inline_response_200::InlineResponse200;
mod inline_response_200_1;
pub use self::inline_response_200_1::InlineResponse2001;
mod inline_response_200_10;
pub use self::inline_response_200_10::InlineResponse20010;
mod inline_response_200_11;
pub use self::inline_response_200_11::InlineResponse20011;
mod inline_response_200_12;
pub use self::inline_response_200_12::InlineResponse20012;
mod inline_response_200_12_actor;
pub use self::inline_response_200_12_actor::InlineResponse20012Actor;
mod inline_response_200_13;
pub use self::inline_response_200_13::InlineResponse20013;
mod inline_response_200_14;
pub use self::inline_response_200_14::InlineResponse20014;
mod inline_response_200_15;
pub use self::inline_response_200_15::InlineResponse20015;
mod inline_response_200_16;
pub use self::inline_response_200_16::InlineResponse20016;
mod inline_response_200_17;
pub use self::inline_response_200_17::InlineResponse20017;
mod inline_response_200_18;
pub use self::inline_response_200_18::InlineResponse20018;
mod inline_response_200_19;
pub use self::inline_response_200_19::InlineResponse20019;
mod inline_response_200_2;
pub use self::inline_response_200_2::InlineResponse2002;
mod inline_response_200_20;
pub use self::inline_response_200_20::InlineResponse20020;
mod inline_response_200_20_descriptor;
pub use self::inline_response_200_20_descriptor::InlineResponse20020Descriptor;
mod inline_response_200_20_platforms;
pub use self::inline_response_200_20_platforms::InlineResponse20020Platforms;
mod inline_response_200_3;
pub use self::inline_response_200_3::InlineResponse2003;
mod inline_response_200_4;
pub use self::inline_response_200_4::InlineResponse2004;
mod inline_response_200_5;
pub use self::inline_response_200_5::InlineResponse2005;
mod inline_response_200_6;
pub use self::inline_response_200_6::InlineResponse2006;
mod inline_response_200_7;
pub use self::inline_response_200_7::InlineResponse2007;
mod inline_response_200_8;
pub use self::inline_response_200_8::InlineResponse2008;
mod inline_response_200_9;
pub use self::inline_response_200_9::InlineResponse2009;
mod inline_response_200_state;
pub use self::inline_response_200_state::InlineResponse200State;
mod inline_response_201;
pub use self::inline_response_201::InlineResponse201;
mod inline_response_201_1;
pub use self::inline_response_201_1::InlineResponse2011;
mod inline_response_201_2;
pub use self::inline_response_201_2::InlineResponse2012;
mod inline_response_201_3;
pub use self::inline_response_201_3::InlineResponse2013;
mod inline_response_201_4;
pub use self::inline_response_201_4::InlineResponse2014;
mod ipam;
pub use self::ipam::Ipam;
mod join_tokens;
pub use self::join_tokens::JoinTokens;
mod local_node_state;
pub use self::local_node_state::LocalNodeState;
mod manager_status;
pub use self::manager_status::ManagerStatus;
mod mount;
pub use self::mount::Mount;
mod mount_bind_options;
pub use self::mount_bind_options::MountBindOptions;
mod mount_point;
pub use self::mount_point::MountPoint;
mod mount_tmpfs_options;
pub use self::mount_tmpfs_options::MountTmpfsOptions;
mod mount_volume_options;
pub use self::mount_volume_options::MountVolumeOptions;
mod mount_volume_options_driver_config;
pub use self::mount_volume_options_driver_config::MountVolumeOptionsDriverConfig;
mod network;
pub use self::network::Network;
mod network_config;
pub use self::network_config::NetworkConfig;
mod network_container;
pub use self::network_container::NetworkContainer;
mod network_settings;
pub use self::network_settings::NetworkSettings;
mod node;
pub use self::node::Node;
mod node_description;
pub use self::node_description::NodeDescription;
mod node_spec;
pub use self::node_spec::NodeSpec;
mod node_state;
pub use self::node_state::NodeState;
mod node_status;
pub use self::node_status::NodeStatus;
mod object_version;
pub use self::object_version::ObjectVersion;
mod peer_node;
pub use self::peer_node::PeerNode;
mod platform;
pub use self::platform::Platform;
mod plugin;
pub use self::plugin::Plugin;
mod plugin_config;
pub use self::plugin_config::PluginConfig;
mod plugin_config_args;
pub use self::plugin_config_args::PluginConfigArgs;
mod plugin_config_interface;
pub use self::plugin_config_interface::PluginConfigInterface;
mod plugin_config_linux;
pub use self::plugin_config_linux::PluginConfigLinux;
mod plugin_config_network;
pub use self::plugin_config_network::PluginConfigNetwork;
mod plugin_config_rootfs;
pub use self::plugin_config_rootfs::PluginConfigRootfs;
mod plugin_config_user;
pub use self::plugin_config_user::PluginConfigUser;
mod plugin_device;
pub use self::plugin_device::PluginDevice;
mod plugin_env;
pub use self::plugin_env::PluginEnv;
mod plugin_interface_type;
pub use self::plugin_interface_type::PluginInterfaceType;
mod plugin_mount;
pub use self::plugin_mount::PluginMount;
mod plugin_settings;
pub use self::plugin_settings::PluginSettings;
mod plugins_info;
pub use self::plugins_info::PluginsInfo;
mod port;
pub use self::port::Port;
mod port_binding;
pub use self::port_binding::PortBinding;
mod port_map;
pub use self::port_map::PortMap;
mod process_config;
pub use self::process_config::ProcessConfig;
mod progress_detail;
pub use self::progress_detail::ProgressDetail;
mod push_image_info;
pub use self::push_image_info::PushImageInfo;
mod reachability;
pub use self::reachability::Reachability;
mod registry_service_config;
pub use self::registry_service_config::RegistryServiceConfig;
mod resource_object;
pub use self::resource_object::ResourceObject;
mod resources;
pub use self::resources::Resources;
mod resources_blkio_weight_device;
pub use self::resources_blkio_weight_device::ResourcesBlkioWeightDevice;
mod resources_ulimits;
pub use self::resources_ulimits::ResourcesUlimits;
mod restart_policy;
pub use self::restart_policy::RestartPolicy;
mod runtime;
pub use self::runtime::Runtime;
mod secret;
pub use self::secret::Secret;
mod secret_spec;
pub use self::secret_spec::SecretSpec;
mod service;
pub use self::service::Service;
mod service_endpoint;
pub use self::service_endpoint::ServiceEndpoint;
mod service_endpoint_virtual_i_ps;
pub use self::service_endpoint_virtual_i_ps::ServiceEndpointVirtualIPs;
mod service_spec;
pub use self::service_spec::ServiceSpec;
mod service_spec_mode;
pub use self::service_spec_mode::ServiceSpecMode;
mod service_spec_mode_replicated;
pub use self::service_spec_mode_replicated::ServiceSpecModeReplicated;
mod service_spec_rollback_config;
pub use self::service_spec_rollback_config::ServiceSpecRollbackConfig;
mod service_spec_update_config;
pub use self::service_spec_update_config::ServiceSpecUpdateConfig;
mod service_update_response;
pub use self::service_update_response::ServiceUpdateResponse;
mod service_update_status;
pub use self::service_update_status::ServiceUpdateStatus;
mod swarm_info;
pub use self::swarm_info::SwarmInfo;
mod swarm_spec;
pub use self::swarm_spec::SwarmSpec;
mod swarm_spec_ca_config;
pub use self::swarm_spec_ca_config::SwarmSpecCaConfig;
mod swarm_spec_ca_config_external_c_as;
pub use self::swarm_spec_ca_config_external_c_as::SwarmSpecCaConfigExternalCAs;
mod swarm_spec_dispatcher;
pub use self::swarm_spec_dispatcher::SwarmSpecDispatcher;
mod swarm_spec_encryption_config;
pub use self::swarm_spec_encryption_config::SwarmSpecEncryptionConfig;
mod swarm_spec_orchestration;
pub use self::swarm_spec_orchestration::SwarmSpecOrchestration;
mod swarm_spec_raft;
pub use self::swarm_spec_raft::SwarmSpecRaft;
mod swarm_spec_task_defaults;
pub use self::swarm_spec_task_defaults::SwarmSpecTaskDefaults;
mod swarm_spec_task_defaults_log_driver;
pub use self::swarm_spec_task_defaults_log_driver::SwarmSpecTaskDefaultsLogDriver;
mod system_info;
pub use self::system_info::SystemInfo;
mod task;
pub use self::task::Task;
mod task_spec;
pub use self::task_spec::TaskSpec;
mod task_spec_container_spec;
pub use self::task_spec_container_spec::TaskSpecContainerSpec;
mod task_spec_container_spec_configs;
pub use self::task_spec_container_spec_configs::TaskSpecContainerSpecConfigs;
mod task_spec_container_spec_dns_config;
pub use self::task_spec_container_spec_dns_config::TaskSpecContainerSpecDnsConfig;
mod task_spec_container_spec_file;
pub use self::task_spec_container_spec_file::TaskSpecContainerSpecFile;
mod task_spec_container_spec_file_1;
pub use self::task_spec_container_spec_file_1::TaskSpecContainerSpecFile1;
mod task_spec_container_spec_privileges;
pub use self::task_spec_container_spec_privileges::TaskSpecContainerSpecPrivileges;
mod task_spec_container_spec_privileges_credential_spec;
pub use self::task_spec_container_spec_privileges_credential_spec::TaskSpecContainerSpecPrivilegesCredentialSpec;
mod task_spec_container_spec_privileges_se_linux_context;
pub use self::task_spec_container_spec_privileges_se_linux_context::TaskSpecContainerSpecPrivilegesSeLinuxContext;
mod task_spec_container_spec_secrets;
pub use self::task_spec_container_spec_secrets::TaskSpecContainerSpecSecrets;
mod task_spec_log_driver;
pub use self::task_spec_log_driver::TaskSpecLogDriver;
mod task_spec_networks;
pub use self::task_spec_networks::TaskSpecNetworks;
mod task_spec_placement;
pub use self::task_spec_placement::TaskSpecPlacement;
mod task_spec_placement_preferences;
pub use self::task_spec_placement_preferences::TaskSpecPlacementPreferences;
mod task_spec_placement_spread;
pub use self::task_spec_placement_spread::TaskSpecPlacementSpread;
mod task_spec_plugin_spec;
pub use self::task_spec_plugin_spec::TaskSpecPluginSpec;
mod task_spec_resources;
pub use self::task_spec_resources::TaskSpecResources;
mod task_spec_restart_policy;
pub use self::task_spec_restart_policy::TaskSpecRestartPolicy;
mod task_state;
pub use self::task_state::TaskState;
mod task_status;
pub use self::task_status::TaskStatus;
mod task_status_container_status;
pub use self::task_status_container_status::TaskStatusContainerStatus;
mod throttle_device;
pub use self::throttle_device::ThrottleDevice;
mod tls_info;
pub use self::tls_info::TlsInfo;
mod volume;
pub use self::volume::Volume;
mod volume_config;
pub use self::volume_config::VolumeConfig;
mod volume_usage_data;
pub use self::volume_usage_data::VolumeUsageData;
mod config_create_body;
pub use self::config_create_body::ConfigCreateBody;
mod container_create_body;
pub use self::container_create_body::ContainerCreateBody;
mod container_update_update;
pub use self::container_update_update::ContainerUpdateUpdate;
mod host_config;
pub use self::host_config::HostConfig;
mod secret_create_body;
pub use self::secret_create_body::SecretCreateBody;
mod swarm;
pub use self::swarm::Swarm;

// TODO(farcaller): sort out files
pub struct File;

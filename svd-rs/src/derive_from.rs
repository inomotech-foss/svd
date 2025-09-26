//! Implementations of DeriveFrom, setting non-explicit fields.
use crate::{
    ClusterInfo,
    DimElement,
    EnumeratedValues,
    FieldInfo,
    MaybeArray,
    PeripheralInfo,
    RegisterInfo,
    RegisterProperties,
};

/// Fill empty fields of structure with values of other structure
pub trait DeriveFrom {
    /// Derive contents
    fn derive_from(&self, other: &Self) -> Self;
}

impl DeriveFrom for ClusterInfo {
    fn derive_from(&self, other: &Self) -> Self {
        // "Remarks: When deriving a cluster, it is mandatory to specify at least the
        // <name>, the <description>, and the <addressOffset>."
        let Self {
            name: _,
            description: _,
            address_offset: _,
            alternate_cluster,
            header_struct_name,
            default_register_properties,
            children,
            derived_from: _,
        } = other;

        let mut derived = self.clone();
        derived.alternate_cluster = derived
            .alternate_cluster
            .or_else(|| alternate_cluster.clone());
        derived.default_register_properties = derived
            .default_register_properties
            .derive_from(default_register_properties);
        derived.header_struct_name = derived
            .header_struct_name
            .or_else(|| header_struct_name.clone());
        if derived.children.is_empty() {
            derived.children = children.clone();
        }
        derived
    }
}

impl DeriveFrom for EnumeratedValues {
    fn derive_from(&self, other: &Self) -> Self {
        let Self {
            name: _,
            usage,
            derived_from: _,
            values,
        } = other;

        let mut derived = self.clone();
        derived.usage = derived.usage.or(*usage);
        if derived.values.is_empty() {
            derived.values = values.clone();
        }
        derived
    }
}

impl DeriveFrom for PeripheralInfo {
    fn derive_from(&self, other: &Self) -> Self {
        let Self {
            name: _,
            display_name,
            version,
            description,
            alternate_peripheral,
            group_name,
            prepend_to_name,
            append_to_name,
            header_struct_name,
            base_address: _,
            default_register_properties,
            address_block,
            interrupt,
            registers,
            derived_from: _,
        } = other;

        let mut derived = self.clone();
        derived.display_name = derived.display_name.or_else(|| display_name.clone());
        derived.version = derived.version.or_else(|| version.clone());
        derived.description = derived.description.or_else(|| description.clone());
        derived.alternate_peripheral = derived
            .alternate_peripheral
            .or_else(|| alternate_peripheral.clone());
        derived.group_name = derived.group_name.or_else(|| group_name.clone());
        derived.prepend_to_name = derived.prepend_to_name.or_else(|| prepend_to_name.clone());
        derived.append_to_name = derived.append_to_name.or_else(|| append_to_name.clone());
        derived.header_struct_name = derived
            .header_struct_name
            .or_else(|| header_struct_name.clone());
        derived.default_register_properties = derived
            .default_register_properties
            .derive_from(&default_register_properties);
        derived.registers = derived.registers.or_else(|| registers.clone());
        derived.address_block = derived.address_block.or_else(|| address_block.clone());
        if derived.interrupt.is_empty() {
            derived.interrupt = interrupt.clone();
        }
        derived
    }
}

impl DeriveFrom for RegisterInfo {
    fn derive_from(&self, other: &Self) -> Self {
        // "Remarks: When deriving, it is mandatory to specify at least the <name>, the
        // <description>, and the <addressOffset>. "
        let Self {
            name: _,
            description: _,
            address_offset: _,
            display_name,
            alternate_group,
            alternate_register,
            properties,
            datatype,
            modified_write_values,
            write_constraint,
            read_action,
            fields,
            derived_from: _,
        } = other;

        let mut derived = self.clone();
        derived.display_name = derived.display_name.or_else(|| display_name.clone());
        derived.alternate_group = derived.alternate_group.or_else(|| alternate_group.clone());
        derived.alternate_register = derived
            .alternate_register
            .or_else(|| alternate_register.clone());
        derived.properties = derived.properties.derive_from(&properties);
        derived.datatype = derived.datatype.or_else(|| datatype.clone());
        derived.fields = derived.fields.or_else(|| fields.clone());
        derived.modified_write_values = derived.modified_write_values.or(*modified_write_values);
        derived.write_constraint = derived.write_constraint.or(*write_constraint);
        derived.read_action = derived.read_action.or(*read_action);
        derived
    }
}

impl DeriveFrom for RegisterProperties {
    fn derive_from(&self, other: &Self) -> Self {
        let Self {
            size,
            access,
            protection,
            reset_value,
            reset_mask,
        } = *other;

        let mut derived = *self;
        derived.size = derived.size.or(size);
        derived.access = derived.access.or(access);
        derived.protection = derived.protection.or(protection);
        derived.reset_value = derived.reset_value.or(reset_value);
        derived.reset_mask = derived.reset_mask.or(reset_mask);
        derived
    }
}

impl DeriveFrom for FieldInfo {
    fn derive_from(&self, other: &Self) -> Self {
        // "Remarks: When deriving, it is mandatory to specify at least the <name> and
        // <description>."
        let Self {
            name: _,
            description: _,
            bit_range: _, // TODO: derive bit_range.
            access,
            modified_write_values,
            write_constraint,
            read_action,
            enumerated_values,
            derived_from,
        } = other;

        let mut derived = self.clone();
        derived.access = derived.access.or(*access);
        if derived.enumerated_values.is_empty() {
            derived.enumerated_values = enumerated_values.clone();
        }
        derived.write_constraint = derived.write_constraint.or(*write_constraint);
        derived.read_action = derived.read_action.or(*read_action);
        derived.modified_write_values = derived.modified_write_values.or(*modified_write_values);
        derived.derived_from = derived.derived_from.or_else(|| derived_from.clone());
        derived
    }
}

impl DeriveFrom for DimElement {
    fn derive_from(&self, other: &Self) -> Self {
        let Self {
            dim: _,           // mandatory
            dim_increment: _, // mandatory
            dim_index,
            dim_name,
            dim_array_index,
        } = other;

        let mut derived = self.clone();
        derived.dim_index = derived.dim_index.or_else(|| dim_index.clone());
        derived.dim_name = derived.dim_name.or_else(|| dim_name.clone());
        derived.dim_array_index = derived.dim_array_index.or_else(|| dim_array_index.clone());
        derived
    }
}

impl<T> DeriveFrom for MaybeArray<T>
where
    T: DeriveFrom + crate::Name,
{
    fn derive_from(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Single(info), Self::Single(other_info)) => {
                Self::Single(info.derive_from(other_info))
            }
            (Self::Single(info), Self::Array(other_info, other_dim)) => {
                if info.name().contains("%s") {
                    let mut dim = other_dim.clone();
                    dim.dim_name = None;
                    Self::Array(info.derive_from(other_info), dim)
                } else {
                    Self::Single(info.derive_from(other_info))
                }
            }
            (Self::Array(info, dim), Self::Single(other_info)) => {
                Self::Array(info.derive_from(other_info), dim.clone())
            }
            (Self::Array(info, dim), Self::Array(other_info, other_dim)) => {
                Self::Array(info.derive_from(other_info), dim.derive_from(other_dim))
            }
        }
    }
}

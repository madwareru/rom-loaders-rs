use bin_serialization_rs::{Reflectable, Endianness};
use std::io::{Result, Read, ErrorKind};
use crate::shared_types::U32Wrapper;
use num_enum::TryFromPrimitive;

pub mod trigger_enums {
    use num_enum::{TryFromPrimitiveError};
    use std::convert::TryFrom;

    #[derive(num_enum::TryFromPrimitive, Copy, Clone, Debug)]
    #[repr(u32)]
    pub enum GeneralCheckType {
        Unknown,
        GroupUnitCount,
        IsUnitInABox,
        IsUnitInACircle,
        GetUnitParameter,
        IsUnitAlive,
        GetDistanceBetweenUnits,
        GetDistanceFromPointToUnit,
        HowManyUnitsFractionHave,
        IsUnitAttacked,
        GetDiplomacy,
        CheckSack,
        GetDistanceToNearestFractionUnit,
        GetDistanceFromPointToUnitWithItem,
        IsItemInSack,
        Vip,
        CheckVariable,
        HowManyStructuresFractionHave,
        GetStructureHealth,
        Teleport,
        CheckScenarioVariable,
        CheckSubObjective,
        SpellInArea,
        SpellOnUnit,
        IsUnitInPoint
    }
    impl Default for GeneralCheckType {
        fn default() -> Self {
            Self::Unknown
        }
    }

    #[derive(Copy, Clone, Debug)]
    pub enum CheckType {
        General(GeneralCheckType),
        Constant
    }
    impl num_enum::TryFromPrimitive for CheckType {
        type Primitive = u32;
        const NAME: &'static str = "InstanceType";
        fn try_from_primitive(number: Self::Primitive) -> Result<Self, TryFromPrimitiveError<Self>> {
            if number <= 0x1B {
                Ok(Self::General(GeneralCheckType::try_from(number).unwrap()))
            } else {
                match number {
                    0x10002 => Ok(Self::Constant),
                    _ => Err(TryFromPrimitiveError{
                        number
                    })
                }
            }
        }
    }
    impl Default for CheckType {
        fn default() -> Self {
            Self::General(Default::default())
        }
    }

    #[derive(num_enum::TryFromPrimitive, Copy, Clone, Debug)]
    #[repr(u32)]
    pub enum ArgumentType {
        Unknown,
        Number,
        Group,
        Fraction,
        Unit,
        X,
        Y,
        Constant,
        Item,
        Structure
    }
    impl Default for ArgumentType {
        fn default() -> Self {
            ArgumentType::Unknown
        }
    }

    #[derive(num_enum::TryFromPrimitive, Copy, Clone, Debug)]
    #[repr(u32)]
    pub enum CheckOperator {
        Equals,
        NotEquals,
        GreaterThan,
        LowerThan,
        GreaterThanEquals,
        LowerThanEquals,
    }
    impl Default for CheckOperator {
        fn default() -> Self {
            Self::Equals
        }
    }

    #[derive(num_enum::TryFromPrimitive, Copy, Clone, Debug)]
    #[repr(u32)]
    pub enum GeneralInstanceType {
        Unknown,
        IncrementMissionStage,
        SendMessage,
        SetVariableValue,
        ForceMissionComplete,
        ForceMissionFailed,
        Command,
        KeepFormation,
        IncrementVariable,
        SetDiplomacy,
        GiveItem,
        AddItemInUnitsSack,
        RemoveItemFromUnitsSack,
        HideUnit,
        ShowUnit,
        MetamorphUnit,
        ChangeUnitsOwner,
        DropAll,
        MagicOnArea,
        ChangeGroupsOwner,
        GiveMoneyToFraction,
        MagicOnUnit,
        CreateMagicTrigger,
        SetStructureHealth,
        MoveUnitImmediate,
        GiveAllItemsFromUnitToUnit,
        TimedSpellOnGround,
        ChangeRespawnTime,
        HideGroup,
        ShowGroup,
        SetUnitsParameter,
        SetScenarioVariable,
        SetSubObjective,
        SetMusicOrder,
        RemoveItemFromAll,
        StopGroup,
    }
    impl Default for GeneralInstanceType {
        fn default() -> Self {
            Self::Unknown
        }
    }

    #[derive(Copy, Clone, Debug)]
    pub enum InstanceType {
        General(GeneralInstanceType),
        StartHere,
        RespawnGroup,
        ChangeMusicTo
    }
    impl num_enum::TryFromPrimitive for InstanceType {
        type Primitive = u32;
        const NAME: &'static str = "InstanceType";
        fn try_from_primitive(number: Self::Primitive) -> Result<Self, TryFromPrimitiveError<Self>> {
            if number <= 0x27 {
                Ok(Self::General(GeneralInstanceType::try_from(number).unwrap()))
            } else {
                match number {
                    0x10002 => Ok(Self::StartHere),
                    0x10003 => Ok(Self::RespawnGroup),
                    0x10004 => Ok(Self::ChangeMusicTo),
                    _ => Err(TryFromPrimitiveError{
                        number
                    })
                }
            }
        }
    }
    impl Default for InstanceType {
        fn default() -> Self {
            Self::General(Default::default())
        }
    }
}

#[derive(Clone, Debug)]
pub struct InstanceEntry {
    pub name: String,
    pub instance_type: trigger_enums::InstanceType,
    pub id: u32,
    pub execute_once: u32,
    pub argument_values: [u32; 10],
    pub argument_types: Vec<trigger_enums::ArgumentType>,
    pub argument_names: Vec<String>
}
impl InstanceEntry {
    pub fn read_from_stream<TStream: Read>(stream: &mut TStream, endianness: Endianness) -> Result<Self> {
        let mut name_buffer = [0u8; 0x40];
        stream.read(&mut name_buffer)?;
        let name = cp866_rs::decode_bytes(&name_buffer);
        let instance_type = trigger_enums::InstanceType::try_from_primitive(
            *U32Wrapper::deserialize(stream, endianness)?
        ).map_err(|_| std::io::Error::from(ErrorKind::InvalidInput))?;
        let id = *(U32Wrapper::deserialize(stream, endianness))?;
        let execute_once = *(U32Wrapper::deserialize(stream, endianness))?;
        let mut argument_values = [0u32; 10];
        let mut argument_types = vec![Default::default(); 10];
        let mut argument_names = Vec::with_capacity(10);
        for value in argument_values.iter_mut() {
            *value = *(U32Wrapper::deserialize(stream, endianness))?;
        }
        for value in argument_types.iter_mut() {
            *value = trigger_enums::ArgumentType::try_from_primitive(
                *U32Wrapper::deserialize(stream, endianness)?
            ).map_err(|_| std::io::Error::from(ErrorKind::InvalidInput))?;
        }
        for _ in 0..10 {
            stream.read(&mut name_buffer)?;
            argument_names.push(cp866_rs::decode_bytes(&name_buffer));
        }
        Ok(Self {
            name,
            instance_type,
            id,
            execute_once,
            argument_values,
            argument_types,
            argument_names
        })
    }
}

#[derive(Clone, Debug)]
pub struct CheckEntry {
    pub name: String,
    pub check_type: trigger_enums::CheckType,
    pub id: u32,
    pub execute_once: u32,
    pub argument_values: [u32; 10],
    pub argument_types: Vec<trigger_enums::ArgumentType>,
    pub argument_names: Vec<String>
}
impl CheckEntry {
    pub fn read_from_stream<TStream: Read>(stream: &mut TStream, endianness: Endianness) -> Result<Self> {
        let mut name_buffer = [0u8; 0x40];
        stream.read(&mut name_buffer)?;
        let name = cp866_rs::decode_bytes(&name_buffer);
        let check_type = trigger_enums::CheckType::try_from_primitive(
            *U32Wrapper::deserialize(stream, endianness)?
        ).map_err(|_| std::io::Error::from(ErrorKind::InvalidInput))?;
        let id = *(U32Wrapper::deserialize(stream, endianness))?;
        let execute_once = *(U32Wrapper::deserialize(stream, endianness))?;
        let mut argument_values = [0u32; 10];
        let mut argument_types = vec![Default::default(); 10];
        let mut argument_names = Vec::with_capacity(10);
        for value in argument_values.iter_mut() {
            *value = *(U32Wrapper::deserialize(stream, endianness))?;
        }
        for value in argument_types.iter_mut() {
            *value = trigger_enums::ArgumentType::try_from_primitive(
                *U32Wrapper::deserialize(stream, endianness)?
            ).map_err(|_| std::io::Error::from(ErrorKind::InvalidInput))?;
        }
        for _ in 0..10 {
            stream.read(&mut name_buffer)?;
            argument_names.push(cp866_rs::decode_bytes(&name_buffer));
        }
        Ok(Self {
            name,
            check_type,
            id,
            execute_once,
            argument_values,
            argument_types,
            argument_names
        })
    }
}

#[derive(Clone, Debug)]
pub struct TriggerEntry {
    pub name: String,
    pub check_identifiers: [u32; 6],
    pub instance_identifiers: [u32; 4],
    pub check_01_operator: Option<trigger_enums::CheckOperator>,
    pub check_23_operator: Option<trigger_enums::CheckOperator>,
    pub check_45_operator: Option<trigger_enums::CheckOperator>,
    pub run_once: u32
}
impl TriggerEntry {
    pub fn read_from_stream<TStream: Read>(stream: &mut TStream, endianness: Endianness) -> Result<Self> {
        let name = {
            let mut name_buffer = [0u8; 0x80];
            stream.read(&mut name_buffer)?;
            cp866_rs::decode_bytes(&name_buffer)
        };
        let mut check_identifiers = [0u32; 6];
        let mut instance_identifiers = [0u32; 4];
        for value in check_identifiers.iter_mut() {
            *value = *(U32Wrapper::deserialize(stream, endianness))?;
        }
        for value in instance_identifiers.iter_mut() {
            *value = *(U32Wrapper::deserialize(stream, endianness))?;
        }
        let check_01_operator = *U32Wrapper::deserialize(stream, endianness)?;
        let check_23_operator = *U32Wrapper::deserialize(stream, endianness)?;
        let check_45_operator = *U32Wrapper::deserialize(stream, endianness)?;

        let check_01_operator = if check_01_operator == 0xFFFFFFFF || check_identifiers[0] == 0 || check_identifiers[1] == 0 {
            None
        } else {
            Some(trigger_enums::CheckOperator::try_from_primitive(
                check_01_operator
            ).map_err(|_| std::io::Error::from(ErrorKind::InvalidInput))?)
        };
        let check_23_operator = if check_23_operator == 0xFFFFFFFF || check_identifiers[2] == 0 || check_identifiers[3] == 0 {
            None
        } else {
            Some(trigger_enums::CheckOperator::try_from_primitive(
                check_23_operator
            ).map_err(|_| std::io::Error::from(ErrorKind::InvalidInput))?)
        };
        let check_45_operator = if check_45_operator == 0xFFFFFFFF || check_identifiers[4] == 0 || check_identifiers[5] == 0 {
            None
        } else {
            Some(trigger_enums::CheckOperator::try_from_primitive(
                check_45_operator
            ).map_err(|_| std::io::Error::from(ErrorKind::InvalidInput))?)
        };
        let run_once = *(U32Wrapper::deserialize(stream, endianness))?;
        Ok(Self {
            name,
            check_identifiers,
            instance_identifiers,
            check_01_operator,
            check_23_operator,
            check_45_operator,
            run_once
        })
    }
}

#[derive(Debug)]
pub struct TriggersSection {
    pub instances: Vec<InstanceEntry>,
    pub checks: Vec<CheckEntry>,
    pub triggers: Vec<TriggerEntry>
}
impl TriggersSection {
    pub fn read_from_stream<TStream: Read>(stream: &mut TStream, endianness: Endianness) -> Result<Self> {
        let instance_count = *(U32Wrapper::deserialize(stream, endianness))?;
        let mut instances = Vec::with_capacity(instance_count as usize);
        for _ in 0..instance_count {
            instances.push(InstanceEntry::read_from_stream(stream, endianness)?);
        }
        let check_count = *(U32Wrapper::deserialize(stream, endianness))?;
        let mut checks = Vec::with_capacity(check_count as usize);
        for _ in 0..check_count {
            checks.push(CheckEntry::read_from_stream(stream, endianness)?);
        }
        let trigger_count = *(U32Wrapper::deserialize(stream, endianness))?;
        let mut triggers = Vec::with_capacity(trigger_count as usize);
        for _ in 0..trigger_count {
            triggers.push(TriggerEntry::read_from_stream(stream, endianness)?);
        }
        Ok(Self { instances, checks, triggers })
    }
}
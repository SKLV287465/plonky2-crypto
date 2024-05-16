use std::marker::PhantomData;

use plonky2::{
    field::extension::Extendable,
    gadgets::{
        arithmetic::EqualityGenerator,
        arithmetic_extension::QuotientGeneratorExtension,
        range_check::LowHighGenerator,
        split_base::BaseSumGenerator,
        split_join::{SplitGenerator, WireSplitGenerator},
    },
    gates::{
        arithmetic_base::{ArithmeticBaseGenerator, ArithmeticGate},
        arithmetic_extension::{ArithmeticExtensionGate, ArithmeticExtensionGenerator},
        base_sum::{BaseSplitGenerator, BaseSumGate},
        constant::ConstantGate,
        coset_interpolation::{CosetInterpolationGate, InterpolationGenerator},
        exponentiation::{ExponentiationGate, ExponentiationGenerator},
        lookup::{LookupGate, LookupGenerator},
        lookup_table::{LookupTableGate, LookupTableGenerator},
        multiplication_extension::{MulExtensionGate, MulExtensionGenerator},
        noop::NoopGate,
        poseidon::{PoseidonGate, PoseidonGenerator},
        poseidon_mds::{PoseidonMdsGate, PoseidonMdsGenerator},
        public_input::PublicInputGate,
        random_access::{RandomAccessGate, RandomAccessGenerator},
        reducing::ReducingGate,
        reducing::ReducingGenerator,
        reducing_extension::ReducingExtensionGate,
        reducing_extension::ReducingGenerator as ReducingExtensionGenerator,
    },
    hash::hash_types::RichField,
    impl_gate_serializer, impl_generator_serializer,
    iop::generator::{
        ConstantGenerator, CopyGenerator, NonzeroTestGenerator, RandomValueGenerator,
    },
    plonk::config::{AlgebraicHasher, GenericConfig},
    recursion::dummy_circuit::DummyProofGenerator,
    util::serialization::{GateSerializer, WitnessGeneratorSerializer},
};

pub mod add_many_u32;
pub mod arithmetic_u32;
pub mod comparison;
pub mod interleave_u32;
pub mod range_check_u32;
pub mod subtraction_u32;
pub mod uninterleave_to_b32;
pub mod uninterleave_to_u32;
use self::{
    add_many_u32::U32AddManyGate,
    arithmetic_u32::U32ArithmeticGenerator,
    comparison::ComparisonGenerator,
    interleave_u32::{U32InterleaveGate, U32InterleaveGenerator},
    range_check_u32::{U32RangeCheckGate, U32RangeCheckGenerator},
    subtraction_u32::{U32SubtractionGate, U32SubtractionGenerator},
    uninterleave_to_b32::{UninterleaveToB32Gate, UninterleaveToB32Generator},
    uninterleave_to_u32::{UninterleaveToU32Gate, UninterleaveToU32Generator},
};
use super::arithmetic_u32::SplitToU32Generator;
use crate::biguint::BigUintDivRemGenerator;
use crate::u32::gates::add_many_u32::U32AddManyGenerator;
use crate::u32::gates::arithmetic_u32::U32ArithmeticGate;
use crate::u32::gates::comparison::ComparisonGate;
use plonky2::get_gate_tag_impl;
use plonky2::get_generator_tag_impl;
use plonky2::read_gate_impl;
use plonky2::read_generator_impl;
use std::default::Default;
pub struct HashGeneratorSerializer<C: GenericConfig<D>, const D: usize> {
    pub _phantom: PhantomData<C>,
}

impl<F, C, const D: usize> WitnessGeneratorSerializer<F, D> for HashGeneratorSerializer<C, D>
where
    F: RichField + Extendable<D>,
    C: GenericConfig<D, F = F> + 'static,
    C::Hasher: AlgebraicHasher<F>,
{
    impl_generator_serializer! {
        DefaultGeneratorSerializer,
        ArithmeticBaseGenerator<F, D>,
        ArithmeticExtensionGenerator<F, D>,
        BaseSplitGenerator<2>,
        BaseSumGenerator<2>,
        ConstantGenerator<F>,
        CopyGenerator,
        DummyProofGenerator<F, C, D>,
        EqualityGenerator,
        ExponentiationGenerator<F, D>,
        InterpolationGenerator<F, D>,
        LookupGenerator,
        LookupTableGenerator,
        LowHighGenerator,
        MulExtensionGenerator<F, D>,
        NonzeroTestGenerator,
        PoseidonGenerator<F, D>,
        PoseidonMdsGenerator<D>,
        QuotientGeneratorExtension<D>,
        RandomAccessGenerator<F, D>,
        RandomValueGenerator,
        ReducingGenerator<D>,
        ReducingExtensionGenerator<D>,
        SplitGenerator,
        WireSplitGenerator,
        // hash generators added
        BigUintDivRemGenerator<F,D>,
        SplitToU32Generator<F, D>,
        U32AddManyGenerator<F,D>,
        U32ArithmeticGenerator<F,D>,
        ComparisonGenerator<F,D>,
        U32InterleaveGenerator,
        U32RangeCheckGenerator<F,D>,
        U32SubtractionGenerator<F,D>,
        UninterleaveToB32Generator,
        UninterleaveToU32Generator
    }
}

pub struct HashGateSerializer;
impl<F: RichField + Extendable<D>, const D: usize> GateSerializer<F, D> for HashGateSerializer {
    impl_gate_serializer! {
        DefaultGateSerializer,
        ArithmeticGate,
        ArithmeticExtensionGate<D>,
        BaseSumGate<2>,
        ConstantGate,
        CosetInterpolationGate<F, D>,
        ExponentiationGate<F, D>,
        LookupGate,
        LookupTableGate,
        MulExtensionGate<D>,
        NoopGate,
        PoseidonMdsGate<F, D>,
        PoseidonGate<F, D>,
        PublicInputGate,
        RandomAccessGate<F, D>,
        ReducingExtensionGate<D>,
        ReducingGate<D>,
        //hash gates
        U32AddManyGate<F,D>,
        U32ArithmeticGate<F, D>,
        ComparisonGate<F, D>,
        U32InterleaveGate,
        U32RangeCheckGate<F,D>,
        U32SubtractionGate<F,D>,
        UninterleaveToB32Gate,
        UninterleaveToU32Gate
    }
}

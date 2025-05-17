import { Module } from '@nestjs/common';
import { ConfigModule, ConfigService } from '@nestjs/config';
import { DpsModule } from './dps/dps.module';
import { RegistryModule } from './registry/registry.module';
import { IngestModule } from './ingest/ingest.module';
import { SolanaModule } from './solana/solana.module';
import { WalrusModule } from './walrus/walrus.module';
import { MongooseModule } from '@nestjs/mongoose';
import { BrokerModule } from './broker/broker.module';
import { ReadingModule }  from './reading/reading.module';
import { ListingModule } from './listing/listing.module';
import configuration from './configuration';

@Module({
  imports: [
    ConfigModule.forRoot({
      isGlobal: true,
      load: [() => require('./configuration').default()],
    }),
    MongooseModule.forRootAsync({
      imports: [ConfigModule],
      useFactory: (configService: ConfigService) => ({
        uri: configService.get<string>('MONGODB_URI'),
      }),
      inject: [ConfigService],
    }),
    SolanaModule,
    WalrusModule,
    DpsModule,
    RegistryModule,
    IngestModule,
    BrokerModule,
    ReadingModule,
    ListingModule,
  ],
})
export class AppModule {}

"use client";

import Heading from "@/components/Heading";
import Text from "@/components/Text";
import View from "@/components/View";
import PageLayout from "@/components/layout/PageLayout";
export default function ConnectionPage() {
  return (
    <>
      <PageLayout pageTitle={"Connect Device"} includeSearchBar={false}>
        <View className=" text-center flex flex-col justify-center items-center h-[500px]">
          <View>
            <Heading className=" text-gray-700 text-2xl">
              Provide Client ID
            </Heading>
            <Text className="mb-8 mt-1  leading-1">
              What is the client id displayed on the peer device?
            </Text>
          </View>

          <View>
            <form action="">
              <View className="w-full">
                <label htmlFor="clientId" className="block sr-only">
                  Client ID
                </label>
                <input
                  type="text"
                  name="clinetId"
                  id=""
                  className="py-3 px-4 rounded-lg  bg-gray-50 text-center form-input w-[300px]"
                  autoFocus
                />
              </View>
            </form>
          </View>
        </View>
      </PageLayout>
    </>
  );
}
